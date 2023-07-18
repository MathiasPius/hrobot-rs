use std::{collections::HashMap, net::Ipv4Addr};

use serde::Deserialize;

use crate::{error::Error, AsyncHttpClient, AsyncRobot};

use super::{
    wrapper::{List, Single},
    UnauthenticatedRequest,
};

fn list_ips() -> UnauthenticatedRequest<List<InternalIp>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/ip")
}

fn get_ip(ip: Ipv4Addr) -> UnauthenticatedRequest<Single<Ip>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/ip/{ip}"))
}

impl<Client: AsyncHttpClient> AsyncRobot<Client> {
    /// List all single IP addresses, grouped by server they are assigned to.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.list_ips().await.unwrap();
    /// # }
    /// ```
    pub async fn list_ips(&self) -> Result<HashMap<u32, Vec<Ip>>, Error> {
        let mut ips: HashMap<u32, Vec<Ip>> = HashMap::new();

        for ip in self.go(list_ips()).await?.0 {
            ips.entry(ip.server_number).or_default().push(ip.inner);
        }

        Ok(ips)
    }

    /// List all single IP addresses, grouped by server they are assigned to.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_ip("123.123.123.123".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn get_ip(&self, ip: Ipv4Addr) -> Result<Ip, Error> {
        Ok(self.go(get_ip(ip)).await?.0)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "InternalTrafficWarnings")]
pub struct TrafficWarnings {
    /// Hourly traffic warning in *MB*.
    #[serde(rename = "traffic_hourly")]
    pub hourly: u32,

    /// Daily traffic warning in *MB*.
    #[serde(rename = "traffic_daily")]
    pub daily: u32,

    /// Monthly traffic warning in *GB*.
    #[serde(rename = "traffic_monthly")]
    pub monthly: u32,
}

// This is the default configuration for servers,
// as specified by Hetzner.
impl Default for TrafficWarnings {
    fn default() -> Self {
        TrafficWarnings {
            hourly: 200,
            daily: 2000,
            monthly: 20,
        }
    }
}

// This structure is used to deserialize and convert from for traffic warnings,
// yielding None if the traffic warnings are disabled.
#[derive(Debug, Clone, Deserialize)]
struct InternalTrafficWarnings {
    traffic_warnings: bool,
    traffic_hourly: u32,
    traffic_daily: u32,
    traffic_monthly: u32,
}

impl TryFrom<InternalTrafficWarnings> for TrafficWarnings {
    type Error = &'static str;

    fn try_from(value: InternalTrafficWarnings) -> Result<Self, Self::Error> {
        if value.traffic_warnings {
            Ok(TrafficWarnings {
                hourly: value.traffic_hourly,
                daily: value.traffic_daily,
                monthly: value.traffic_monthly,
            })
        } else {
            Err("traffic warnings disabled")
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Network {
    /// Gateway for the IP address.
    pub gateway: Ipv4Addr,

    /// Netmask for the IP address.
    pub mask: u8,

    /// Broadcast address for the IP address.
    pub broadcast: Ipv4Addr,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ip {
    /// Address
    pub ip: Ipv4Addr,

    /// Status of locking.
    pub locked: bool,

    /// Network subnet this IP address exists in.
    ///
    /// This field is only available when retrieving the address directly ([`AsyncRobot::list_ips()`])
    /// and is not returned when listing IP addresses generally ([`AsyncRobot::get_ip()`])
    #[serde(flatten)]
    pub network: Option<Network>,

    /// Separate MAC address, if any.
    pub separate_mac: Option<String>,

    /// Traffic warnings for this IP address.
    #[serde(flatten)]
    pub traffic_warnings: Option<TrafficWarnings>,
}

#[derive(Deserialize)]
struct InternalIp {
    pub server_number: u32,

    #[serde(flatten)]
    pub inner: Ip,
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_list_ips() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();
        let ips = robot.list_ips().await.unwrap();

        info!("{ips:#?}");
    }

    #[tokio::test]
    #[traced_test]
    async fn test_get_server_ip_information() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(ip) = servers.into_iter().find_map(|server| server.ipv4) {
            let ip = robot.get_ip(ip).await.unwrap();
            info!("{ip:#?}");
        }
    }
}
