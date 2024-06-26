use bytesize::ByteSize;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};
use time::Date;

/// Unique Server ID.
///
/// Simple wrapper around a u32, to avoid confusion with for example [`TemplateId`](crate::api::firewall::TemplateId)
/// and to make it intuitive what kind of argument you need to give to functions like [`AsyncRobot::get_server`](crate::AsyncRobot::get_server()).
///
/// Using a plain integer means it isn't clear what the argument is, is it a counter of my servers, where the argument
/// is in range `0..N` where `N` is the number of dedicated servers in my account, or is it a limiter, like get first `N`
/// servers, for example.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServerId(pub u32);

impl From<u32> for ServerId {
    fn from(value: u32) -> Self {
        ServerId(value)
    }
}

impl From<ServerId> for u32 {
    fn from(value: ServerId) -> Self {
        value.0
    }
}

impl Display for ServerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<u32> for ServerId {
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

/// Indicates the status of a server.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    /// Server is ready for use.
    #[serde(rename = "ready")]
    Ready,
    /// Server is being provisioned or otherwise unavailable.
    #[serde(rename = "in progress")]
    InProgress,
}

/// Reference to a Subnet.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubnetReference {
    /// Subnet Address
    #[serde(rename = "ip")]
    pub ip: IpAddr,

    /// Subnet mask.
    pub mask: String,
}

/// Flags describe availability of a service or add-on for the server.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerFlags {
    /// Server reset is available.
    pub reset: bool,

    /// Server rescue is available.
    pub rescue: bool,

    /// VNC installation is available.
    pub vnc: bool,

    /// Windows installation is available
    pub windows: bool,

    /// Plesk installation is available
    pub plesk: bool,

    /// CPanel installation is available.
    pub cpanel: bool,

    /// Wake-on-LAN is available.
    pub wol: bool,

    /// Hot-swap is available.
    pub hot_swap: bool,

    /// StorageBox this server is linked with (if any).
    pub linked_storagebox: Option<u32>,
}

/// Describes a Hetzner Dedicated Server instance.
#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    /// *Primary* IPv4 address.
    ///
    /// A server can have multiple IPv4 addresses assigned, see [`ips`](Server::ips) and [`subnets`](Server::subnets)
    #[serde(rename = "server_ip")]
    pub ipv4: Option<Ipv4Addr>,

    /// *Primary* IPv6 prefix.
    ///
    /// A server can have multiple IPv6 addresses assigned, see [`ips`](Server::ips) and [`subnets`](Server::subnets)
    #[serde(rename = "server_ipv6_net")]
    pub ipv6_net: Ipv6Addr,

    /// Unique ID of the server.
    #[serde(rename = "server_number")]
    pub id: ServerId,

    /// Name as shown in the Robot interface for the server.
    #[serde(rename = "server_name")]
    pub name: String,

    /// Product name of the server. e.g. `AX41-NVME` or `Server Auction`
    pub product: String,

    /// Datacenter in which the sever is located. e.g. `FSN1-DC14` for Datacenter-14 at Data Center Park Falkenstein.
    ///
    /// See [here](https://www.hetzner.com/unternehmen/rechenzentrum) for a list of datacenters.
    pub dc: String,

    /// Monthly traffic limitation if any, e.g. `5 TB`.
    #[serde(rename = "traffic", deserialize_with = "crate::conversion::traffic")]
    pub traffic_limit: Option<ByteSize>,

    /// Current status of the server.
    pub status: Status,

    /// True if server has been cancelled.
    pub cancelled: bool,

    /// Server has been paid for until this date. Format is `YYYY-MM-DD`.
    pub paid_until: String,

    /// IP Addresses associated with this server.
    ///
    /// Includes both IPv4 and IPv6 but excludes associated subnets,
    /// which are instead listed in [`subnets`](Server::subnets)
    #[serde(
        rename = "ip",
        default,
        deserialize_with = "crate::conversion::deserialize_null_default"
    )]
    pub ips: Vec<String>,

    /// Subnets associated with this server.
    #[serde(rename = "subnet", default)]
    pub subnets: Vec<SubnetReference>,

    /// Server flags indicating availability of extra services.
    ///
    /// This field is only populated when fetching a server directly,
    /// and is not included when listing servers using
    /// [`AsyncRobot::list_servers()`](crate::AsyncRobot::list_servers)
    #[serde(flatten)]
    pub availability: Option<ServerFlags>,
}

/// Describes the terms under which a server was cancelled.
#[derive(Debug, Serialize, Deserialize)]
pub struct Cancelled {
    /// Date on which the cancellation will take effect.
    #[serde(rename = "cancellation_date")]
    pub date: Date,

    /// Reason for the cancellation.
    #[serde(rename = "cancellation_reason")]
    pub reason: Option<String>,

    /// Indicates if the server location will be reserved after server cancellation.
    pub reserved: bool,
}

/// Describes a server cancellation order.
#[derive(Debug)]
pub struct Cancel {
    /// Date on which the cancellation will take effect.
    pub date: Option<Date>,

    /// Reason for the cancellation.
    pub reason: Option<String>,

    /// Indicates if the server location will be reserved after server cancellation.
    pub reserved: bool,
}

#[derive(Serialize)]
pub(crate) struct InternalCancel {
    #[serde(rename = "cancellation_date")]
    pub date: String,
    #[serde(rename = "cancellation_reason")]
    pub reason: Option<String>,
    pub reserved: bool,
}

impl From<Cancel> for InternalCancel {
    fn from(value: Cancel) -> Self {
        InternalCancel {
            date: value
                .date
                .map(|date| date.to_string())
                .unwrap_or("now".to_string()),
            reason: value.reason,
            reserved: value.reserved,
        }
    }
}

/// Describes possibility of cancellation for a server.
#[derive(Debug, Serialize, Deserialize)]
pub struct Cancellable {
    /// Earliest date at which it is possible to
    /// cancel the server.
    pub earliest_cancellation_date: Date,

    /// Indicates whether the current server location
    /// is eligible for reservation after server
    /// cancellation
    pub reservation_possible: bool,

    /// List of possible reasons for cancellations.
    #[serde(rename = "cancellation_reason")]
    pub cancellation_reasons: Vec<String>,
}

/// Indicates the cancellation status of the server.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Cancellation {
    /// Server has been cancelled.
    Cancelled(Cancelled),
    /// Server has not been cancelled.
    Cancellable(Cancellable),
}

#[cfg(test)]
mod tests {
    use crate::api::server::ServerId;

    #[test]
    fn server_id_conversion() {
        assert_eq!(ServerId(10), ServerId::from(10));

        assert_eq!(u32::from(ServerId(10)), 10);
        assert_eq!(ServerId(10), 10);
    }
}
