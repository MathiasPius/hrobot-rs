//! Failover IP/subnet structs and implementation.
use std::net::IpAddr;

use ipnet::IpNet;
use serde::{Deserialize, Serialize};

use crate::{error::Error, AsyncRobot};

use super::{
    server::ServerId,
    wrapper::{List, Single},
    UnauthenticatedRequest,
};

fn list_ips() -> UnauthenticatedRequest<List<InternalFailoverIp>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/failover")
}

fn get_ip(ip: IpAddr) -> UnauthenticatedRequest<Single<InternalFailoverIp>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/failover/{ip}"))
}

fn switch_ip_routing(
    failover_ip: IpAddr,
    target: IpAddr,
) -> Result<UnauthenticatedRequest<Single<InternalFailoverIp>>, serde_html_form::ser::Error> {
    #[derive(Serialize)]
    struct RerouteIp {
        active_server_ip: IpAddr,
    }
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/failover/{failover_ip}"
    ))
    .with_method("POST")
    .with_body(RerouteIp {
        active_server_ip: target,
    })
}

fn disable_routing(failover_ip: IpAddr) -> UnauthenticatedRequest<Single<InternalFailoverIp>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/failover/{failover_ip}"
    ))
    .with_method("DELETE")
}

impl AsyncRobot {
    /// List all failover IP addresses.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.list_failover_ips().await.unwrap();
    /// # }
    /// ```
    pub async fn list_failover_ips(&self) -> Result<Vec<Failover>, Error> {
        Ok(self
            .go(list_ips())
            .await?
            .0
            .into_iter()
            .map(Failover::from)
            .collect())
    }

    /// Get information about a single failover IP address.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_failover_ip("123.123.123.123".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn get_failover_ip(&self, ip: IpAddr) -> Result<Failover, Error> {
        Ok(self.go(get_ip(ip)).await?.0.into())
    }

    /// Switch the routing of the failover IP to instead route to the given target IP.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.switch_failover_routing(
    ///     "2a01:4f8:fff1::".parse().unwrap(),
    ///     "2a01:4f8:0:5176::".parse().unwrap()
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn switch_failover_routing(
        &self,
        failover: IpAddr,
        target: IpAddr,
    ) -> Result<Failover, Error> {
        Ok(self
            .go(switch_ip_routing(failover, target)?)
            .await?
            .0
            .into())
    }

    /// Switch the routing of the failover IP to instead route to the given target IP.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_failover_routing("2a01:4f8:fff1::".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_failover_routing(&self, failover: IpAddr) -> Result<Failover, Error> {
        Ok(self.go(disable_routing(failover)).await?.0.into())
    }
}

/// A failover IP or subnet.
#[derive(Debug, Clone)]
pub struct Failover {
    /// Failover IPv4 or IPv6 address/subnet.
    pub ip: IpNet,

    /// Unique ID of the server to which this address is routed.
    pub active_server: ServerId,

    /// Primary IP Address of the server to which this address is routed.
    ///
    /// Note: If the failover address was assigned to a server and then
    /// had routing disabled, the `active_server` field will still be set,
    /// but this field will be `None`
    pub server_address: Option<IpAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InternalFailoverIp {
    pub ip: IpAddr,
    pub mask: u8,
    pub server_number: ServerId,
    pub active_server_ip: Option<IpAddr>,
}

impl From<InternalFailoverIp> for Failover {
    fn from(value: InternalFailoverIp) -> Self {
        Failover {
            ip: IpNet::new(value.ip, value.mask).unwrap(),
            active_server: value.server_number,
            server_address: value.active_server_ip,
        }
    }
}
