use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug, Deserialize)]
pub enum Status {
    /// Server is ready for use.
    #[serde(rename = "ready")]
    Ready,
    /// Server is being provisioned or otherwise unavailable
    #[serde(rename = "in progress")]
    InProgress,
}

/// Reference to a Subnet.
#[derive(Debug, Deserialize)]
pub struct SubnetReference {
    /// Subnet Address
    #[serde(rename = "ip")]
    pub ip: IpAddr,

    /// Subnet mask.
    pub mask: String,
}

/// Flags describe availability of a service or add-on for the server.
#[derive(Debug, Deserialize)]
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

    // CPanel installation is available.
    pub cpanel: bool,

    /// Wake-on-LAN is available.
    pub wol: bool,

    /// Hot-swap is available.
    pub hot_swap: bool,

    /// StorageBox this server is linked with (if any).
    pub linked_storagebox: Option<u32>,
}

/// Describes a Hetzner Dedicated Server instance.
#[derive(Debug, Deserialize)]
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
    pub id: u32,

    /// Name as shown in the Robot interface for the server.
    #[serde(rename = "server_name")]
    pub name: String,

    /// Product name of the server. e.g. `AX41-NVME` or `Server Auction`
    pub product: String,

    /// Datacenter in which the sever is located. e.g. `FSN1-DC14` for Datacenter-14 at Data Center Park Falkenstein.
    ///
    /// See [here](https://www.hetzner.com/unternehmen/rechenzentrum) for a list of datacenters.
    pub dc: String,

    /// Monthly traffic limitation if any, e.g. `5 TB`, `Unlimited` otherwise.
    pub traffic: String,

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
    #[serde(rename = "ip", default)]
    pub ips: Vec<String>,

    /// Subnets associated with this server.
    #[serde(rename = "subnet", default)]
    pub subnets: Vec<SubnetReference>,

    /// Server flags indicating availability of extra services.
    #[serde(flatten)]
    pub extended: Option<ServerFlags>,
}

/// If the server has been cancelled the struct will reflect this status, otherwise it will
/// contain information about when the earliest possible cancellation is, and whether reserving
/// the server upon cancellation is possible
#[derive(Debug, Deserialize)]
pub struct Cancellation {
    /// Primary IPv4 address of the server.
    #[serde(rename = "server_ip")]
    pub ipv4: Option<Ipv4Addr>,

    /// Primary IPv6 address of the server.
    ///
    /// Note: This field is listed in the API documentation, but I have never
    /// seen it actually get returned from the API.
    #[serde(rename = "server_ipv6_net")]
    pub ipv6: Option<Ipv6Addr>,

    /// Unique ID of the server.
    #[serde(rename = "server_number")]
    pub id: u32,

    /// Name of the server, as it appears in the Hetzner Robot interface.
    #[serde(rename = "server_name")]
    pub name: String,

    /// Earliest point at which the server may be cancelled. Format: YYYY-MM-DD
    pub earliest_cancellation_date: String,

    /// Indicates if the server has already been cancelled.
    pub cancelled: bool,

    /// Indicates whether the current server location is eligible for
    /// reservation after server cancellation.
    pub reservation_possible: bool,

    /// Indicates whether the current server location will be reserved
    /// after server cancellation
    pub reserved: bool,

    /// Cancellation date if the server has been cancelled. Format: YYYY-MM-DD
    pub cancellation_date: Option<String>,

    /// List of possible cancellation reasons, if the server has not been
    /// cancelled, or the single given reason, if it has.
    pub cancellation_reason: Vec<String>,
}
