use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
};

use ipnet::IpNet;
use serde::Deserialize;
use time::Date;

use crate::{error::Error, AsyncHttpClient, AsyncRobot};

use super::{
    ip::{ExecutedMacRemoval, InternalMac, InternalTrafficWarnings, TrafficWarnings},
    server::ServerId,
    wrapper::{List, Single},
    UnauthenticatedRequest,
};

fn list_subnets() -> UnauthenticatedRequest<List<InternalSubnet>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/subnet")
}

fn get_subnet(ip: IpAddr) -> UnauthenticatedRequest<Single<InternalSubnet>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/subnet/{ip}"))
}

fn enable_traffic_warnings(
    ip: IpAddr,
    traffic_warnings: Option<TrafficWarnings>,
) -> Result<UnauthenticatedRequest<Single<Subnet>>, serde_html_form::ser::Error> {
    let request =
        UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/subnet/{ip}"))
            .with_method("POST");

    if let Some(warnings) = traffic_warnings {
        request.with_body(InternalTrafficWarnings::from(warnings))
    } else {
        Ok(request.with_serialized_body("traffic_warnings=true".to_string()))
    }
}

fn disable_traffic_warnings(ip: IpAddr) -> UnauthenticatedRequest<Single<Subnet>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/subnet/{ip}"))
        .with_method("POST")
        .with_serialized_body("traffic_warnings=false".to_string())
}

fn get_separate_mac(ip: IpAddr) -> UnauthenticatedRequest<Single<InternalMac>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/subnet/{ip}/mac"))
}

fn generate_separate_mac(ip: IpAddr) -> UnauthenticatedRequest<Single<InternalMac>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/subnet/{ip}/mac"))
        .with_method("PUT")
}

fn delete_separate_mac(ip: IpAddr) -> UnauthenticatedRequest<Single<ExecutedMacRemoval>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/subnet/{ip}/mac"))
        .with_method("DELETE")
}

fn get_subnet_cancellation(ip: Ipv4Addr) -> UnauthenticatedRequest<Single<Cancellation>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/subnet/{ip}/cancellation"
    ))
}

fn cancel_subnet(ip: Ipv4Addr, date: Date) -> UnauthenticatedRequest<Single<Cancelled>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/subnet/{ip}/cancellation"
    ))
    .with_method("POST")
    .with_serialized_body(format!("cancellation_date={date}"))
}

fn revoke_subnet_cancellation(ip: Ipv4Addr) -> UnauthenticatedRequest<Single<Cancellable>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/subnet/{ip}/cancellation"
    ))
    .with_method("DELETE")
}

impl<Client: AsyncHttpClient> AsyncRobot<Client> {
    /// List all subnets, grouped by server they are assigned to.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.list_subnets().await.unwrap();
    /// # }
    /// ```
    pub async fn list_subnets(&self) -> Result<HashMap<ServerId, Vec<Subnet>>, Error> {
        let mut subnets: HashMap<ServerId, Vec<Subnet>> = HashMap::new();

        for ip in self.go(list_subnets()).await?.0 {
            subnets.entry(ip.server_number).or_default().push(ip.into());
        }

        Ok(subnets)
    }

    // Get subnet information.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_subnet("2a01:4f8:123:123::".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn get_subnet(&self, subnet_addr: IpAddr) -> Result<Subnet, Error> {
        Ok(self.go(get_subnet(subnet_addr)).await?.0.into())
    }

    /// Enable traffic warnings for the subnet, optionally overriding
    /// the existing traffic limits.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::ip::TrafficWarnings;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_ip_traffic_warnings(
    ///     "2a01:4f8:123:123::".parse().unwrap(),
    ///     Some(TrafficWarnings {
    ///         hourly: 200, /* MB */
    ///         daily: 2000, /* MB */
    ///         monthly: 20, /* GB */
    ///     })
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_subnet_traffic_warnings(
        &self,
        ip: IpAddr,
        traffic_warnings: Option<TrafficWarnings>,
    ) -> Result<Subnet, Error> {
        Ok(self
            .go(enable_traffic_warnings(ip, traffic_warnings)?)
            .await?
            .0)
    }

    /// Disable traffic warnings for the subnet.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_subnet_traffic_warnings("2a01:4f8:123:123::".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_subnet_traffic_warnings(&self, ip: IpAddr) -> Result<Subnet, Error> {
        Ok(self.go(disable_traffic_warnings(ip)).await?.0)
    }

    /// Get the separate MAC address for this subnet.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_subnet_separate_mac("2a01:4f8:123:123::".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn get_subnet_separate_mac(&self, ip: IpAddr) -> Result<String, Error> {
        Ok(self.go(get_separate_mac(ip)).await?.0.mac)
    }

    /// Generate a separate MAC address for subnet.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.generate_subnet_separate_mac("2a01:4f8:123:123::".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn generate_subnet_separate_mac(&self, ip: IpAddr) -> Result<String, Error> {
        Ok(self.go(generate_separate_mac(ip)).await?.0.mac)
    }

    /// Remove the separate MAC address for a subnet.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.remove_subnet_separate_mac("2a01:4f8:123:123::".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn remove_subnet_separate_mac(&self, ip: IpAddr) -> Result<(), Error> {
        self.go(delete_separate_mac(ip)).await.map(|_| ())
    }

    /// Get cancellation status for a subnet.
    ///
    /// Note: Only IPv4 subnets can be cancelled.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_subnet_cancellation("123.123.123.123".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn get_subnet_cancellation(&self, ip: Ipv4Addr) -> Result<Cancellation, Error> {
        Ok(self.go(get_subnet_cancellation(ip)).await?.0)
    }

    /// Cancel a subnet.
    ///
    /// Note: Only IPv4 subnets can be cancelled.
    ///
    /// # Example
    /// ```rust,no_run
    /// use time::{Date, Month};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.cancel_subnet(
    ///     "123.123.123.123".parse().unwrap(),
    ///     Date::from_calendar_date(2023, Month::July, 17).unwrap()
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn cancel_subnet(&self, ip: Ipv4Addr, date: Date) -> Result<Cancelled, Error> {
        Ok(self.go(cancel_subnet(ip, date)).await?.0)
    }

    /// Revoke subnet cancellation.
    ///
    /// Note: Only IPv4 subnets can be cancelled.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.revoke_ip_cancellation("123.123.123.123".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn revoke_subnet_cancellation(&self, ip: Ipv4Addr) -> Result<Cancellable, Error> {
        Ok(self.go(revoke_subnet_cancellation(ip)).await?.0)
    }
}

// Used to convert from the plain IP representation provided by Hetzner
// and condensing it into [`ipnet`] structures.
#[derive(Debug, Clone, Deserialize)]
struct InternalSubnet {
    pub server_number: ServerId,
    pub ip: IpAddr,
    pub mask: u8,
    pub gateway: IpAddr,
    pub locked: bool,
    pub failover: bool,
    #[serde(flatten)]
    pub traffic_warnings: Option<TrafficWarnings>,
}

impl From<InternalSubnet> for Subnet {
    fn from(value: InternalSubnet) -> Self {
        Subnet {
            // UNWRAP: Assume prefix lengths given by Hetzner are valid.
            ip: IpNet::new(value.ip, value.mask).unwrap(),
            server_number: value.server_number,
            gateway: value.gateway,
            locked: value.locked,
            failover: value.failover,
            traffic_warnings: value.traffic_warnings,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Subnet {
    /// Address
    pub ip: IpNet,

    /// Server the subnet belongs to
    pub server_number: ServerId,

    /// Gateway address for the subnet.
    pub gateway: IpAddr,

    /// Status of locking.
    pub locked: bool,

    /// True if subnet is a failover subnet
    pub failover: bool,

    /// Traffic warnings for this IP address.
    #[serde(flatten)]
    pub traffic_warnings: Option<TrafficWarnings>,
}

/// IP address has been cancelled.
#[derive(Debug, Clone, Deserialize)]
pub struct Cancelled {
    /// Date at which the IP address is terminated.
    #[serde(rename = "cancellation_date")]
    pub date: Date,
}

/// IP address has not yet been cancelled.
#[derive(Debug, Clone, Deserialize)]
pub struct Cancellable {
    /// Earliest possible date at which the IP address can be cancelled.
    pub earliest_cancellation_date: Date,
}

/// Describes the cancellation state of the IP address.
///
/// If the address has already been cancelled, this contains a [`Cancelled`]
/// otherwise a [`Cancellable`] structure which describes the earliest date
/// at which the IP address can be cancelled.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Cancellation {
    /// IP address has been cancelled.
    Cancelled(Cancelled),
    /// IP address has not yet been cancelled.
    Cancellable(Cancellable),
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use tracing::info;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_list_subnets() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();
        let subnets = robot.list_subnets().await.unwrap();

        info!("{subnets:#?}");
    }

    #[tokio::test]
    #[traced_test]
    async fn test_get_subnets() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();
        let subnets = robot.list_subnets().await.unwrap();
        info!("{subnets:#?}");

        let subnet = subnets
            .values()
            .into_iter()
            .find_map(|subnet| subnet.first());

        if let Some(subnet) = subnet {
            let subnet = robot.get_subnet(subnet.ip.addr()).await.unwrap();
            info!("{subnet:#?}");
        }
    }

    #[tokio::test]
    #[traced_test]
    async fn test_get_subnet_cancellation() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();
        let subnets = robot.list_subnets().await.unwrap();
        info!("{subnets:#?}");

        let subnet = subnets
            .values()
            .into_iter()
            .filter_map(|subnet| subnet.first())
            .find_map(|subnet| match subnet.ip.addr() {
                IpAddr::V4(addr) => Some(addr),
                _ => None,
            });

        if let Some(subnet) = subnet {
            let cancellation = robot.get_subnet_cancellation(subnet).await.unwrap();
            info!("{cancellation:#?}");
        }
    }
}
