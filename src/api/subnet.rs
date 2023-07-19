use std::{collections::HashMap, net::IpAddr};

use ipnet::IpNet;
use serde::Deserialize;
use time::Date;

use crate::{error::Error, AsyncHttpClient, AsyncRobot};

use super::{
    ip::TrafficWarnings,
    wrapper::{List, Single},
    UnauthenticatedRequest,
};

fn list_subnets() -> UnauthenticatedRequest<List<InternalSubnet>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/subnet")
}

fn get_subnet(ip: IpAddr) -> UnauthenticatedRequest<Single<InternalSubnet>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/subnet/{ip}"))
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
    pub async fn list_subnets(&self) -> Result<HashMap<u32, Vec<Subnet>>, Error> {
        let mut subnets: HashMap<u32, Vec<Subnet>> = HashMap::new();

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
}

// Used to convert from the plain IP representation provided by Hetzner
// and condensing it into [`ipnet`] structures.
#[derive(Debug, Clone, Deserialize)]
struct InternalSubnet {
    pub server_number: u32,
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
    pub server_number: u32,

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
}
