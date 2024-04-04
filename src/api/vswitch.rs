//! vSwitch structs and implementation.

use std::{fmt::Display, net::IpAddr};

use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use time::Date;

use crate::{error::Error, urlencode::UrlEncode, AsyncRobot};

use super::{server::ServerId, wrapper::Empty, UnauthenticatedRequest};

fn list_vswitches() -> UnauthenticatedRequest<Vec<VSwitchReference>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/vswitch")
}

fn get_vswitch(vswitch: VSwitchId) -> UnauthenticatedRequest<InternalVSwitch> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/vswitch/{vswitch}"
    ))
}

#[derive(Serialize)]
struct UpdateVSwitch<'a> {
    name: &'a str,
    vlan: u16,
}

fn create_vswitch(
    name: &str,
    vlan_id: VlanId,
) -> Result<UnauthenticatedRequest<VSwitchReference>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/vswitch")
        .with_method("POST")
        .with_body(UpdateVSwitch {
            name,
            vlan: vlan_id.0,
        })
}

fn update_vswitch(
    vswitch_id: VSwitchId,
    name: &str,
    vlan_id: VlanId,
) -> Result<UnauthenticatedRequest<Empty>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/vswitch/{vswitch_id}"
    ))
    .with_method("POST")
    .with_body(UpdateVSwitch {
        name,
        vlan: vlan_id.0,
    })
}

fn delete_vswitch(vswitch_id: VSwitchId, date: Option<Date>) -> UnauthenticatedRequest<Empty> {
    let date = date
        .map(|date| date.to_string())
        .unwrap_or("now".to_string());

    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/vswitch/{vswitch_id}"
    ))
    .with_method("DELETE")
    .with_serialized_body(format!("cancellation_date={date}"))
}

#[derive(Debug, Clone, Serialize)]
struct ServerList<'a> {
    server: &'a [ServerId],
}

impl<'a> UrlEncode for ServerList<'a> {
    fn encode_into(&self, mut f: crate::urlencode::UrlEncodingBuffer<'_>) {
        for server in self.server {
            f.set("server[]", server);
        }
    }
}

fn add_servers(vswitch_id: VSwitchId, servers: &[ServerId]) -> UnauthenticatedRequest<Empty> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/vswitch/{vswitch_id}/server"
    ))
    .with_method("POST")
    .with_serialized_body(ServerList { server: servers }.encode())
}

fn remove_servers(vswitch_id: VSwitchId, servers: &[ServerId]) -> UnauthenticatedRequest<Empty> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/vswitch/{vswitch_id}/server"
    ))
    .with_method("DELETE")
    .with_serialized_body(ServerList { server: servers }.encode())
}

impl AsyncRobot {
    /// List all vSwitches.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.list_vswitches().await.unwrap();
    /// # }
    /// ```
    pub async fn list_vswitches(&self) -> Result<Vec<VSwitchReference>, Error> {
        self.go(list_vswitches()).await
    }

    /// Get vSwitch information.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::vswitch::VSwitchId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_vswitch(VSwitchId(123456)).await.unwrap();
    /// # }
    /// ```
    pub async fn get_vswitch(&self, vswitch: VSwitchId) -> Result<VSwitch, Error> {
        Ok(self.go(get_vswitch(vswitch)).await?.into())
    }

    /// Create a vSwitch
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::vswitch::VlanId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.create_vswitch("vswitch-test-1", VlanId(4078)).await.unwrap();
    /// # }
    /// ```
    pub async fn create_vswitch(
        &self,
        name: &str,
        vlan_id: VlanId,
    ) -> Result<VSwitchReference, Error> {
        self.go(create_vswitch(name, vlan_id)?).await
    }

    /// Update vSwitch.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::vswitch::{VSwitchId, VlanId};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.update_vswitch(
    ///     VSwitchId(124567),
    ///     "vswitch-test-2",
    ///     VlanId(4079)
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn update_vswitch(
        &self,
        vswitch_id: VSwitchId,
        name: &str,
        vlan_id: VlanId,
    ) -> Result<(), Error> {
        self.go(update_vswitch(vswitch_id, name, vlan_id)?)
            .await?
            .throw_away();
        Ok(())
    }

    /// Cancel vSwitch.
    ///
    /// If cancellation date is ommitted, the cancellation is immediate.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::vswitch::VSwitchId;
    /// # use hrobot::time::{Date, Month};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.cancel_vswitch(
    ///     VSwitchId(124567),
    ///     Some(Date::from_calendar_date(2023, Month::July, 10).unwrap())
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn cancel_vswitch(
        &self,
        vswitch_id: VSwitchId,
        cancellation_date: Option<Date>,
    ) -> Result<(), Error> {
        self.go(delete_vswitch(vswitch_id, cancellation_date))
            .await?
            .throw_away();
        Ok(())
    }

    /// Connect dedicated servers to vSwitch.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::vswitch::VSwitchId;
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.connect_vswitch_servers(
    ///     VSwitchId(124567),
    ///     &[ServerId(1234567)],
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn connect_vswitch_servers(
        &self,
        vswitch_id: VSwitchId,
        server_ids: &[ServerId],
    ) -> Result<(), Error> {
        self.go(add_servers(vswitch_id, server_ids))
            .await?
            .throw_away();
        Ok(())
    }

    /// Disconnect dedicated servers from vSwitch.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::vswitch::VSwitchId;
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disconnect_vswitch_servers(
    ///     VSwitchId(124567),
    ///     &[ServerId(1234567)],
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn disconnect_vswitch_servers(
        &self,
        vswitch_id: VSwitchId,
        server_ids: &[ServerId],
    ) -> Result<(), Error> {
        self.go(remove_servers(vswitch_id, server_ids))
            .await?
            .throw_away();
        Ok(())
    }
}

/// VLAN ID.
///
/// Simple wrapper around a u16, to avoid confusion with vSwitch ID, for example.
///
/// VLAN IDs must be in the range 4000..=4091.
///
/// Multiple vSwitches can have the same VLAN ID.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VlanId(pub u16);

impl From<u16> for VlanId {
    fn from(value: u16) -> Self {
        VlanId(value)
    }
}

impl From<VlanId> for u16 {
    fn from(value: VlanId) -> Self {
        value.0
    }
}

impl Display for VlanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<u16> for VlanId {
    fn eq(&self, other: &u16) -> bool {
        self.0.eq(other)
    }
}

/// Uniquely identifies a vSwitch.
///
/// Simple wrapper around a u32, to avoid confusion with other simple integer-based IDs
/// such as [`VlanId`] and to make it intuitive what kind
/// of argument you need to give to functions.
///
/// Using a plain integer means it isn't clear what the argument is, is it a counter of
/// my vSwitches, where the argument is in range `0..N` where `N` is the number of
/// vswitches in my account, or is it a limiter, like get first `N` vswitches, for example.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VSwitchId(pub u32);

impl From<u32> for VSwitchId {
    fn from(value: u32) -> Self {
        VSwitchId(value)
    }
}

impl From<VSwitchId> for u32 {
    fn from(value: VSwitchId) -> Self {
        value.0
    }
}

impl Display for VSwitchId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<u32> for VSwitchId {
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

/// Simplified view of a VSwitch.
///
/// This is returned when [listing](AsyncRobot::list_vswitches()) vSwitches, and only contains
/// the basic vSwitch configuration options. For information on which servers, subnets and
/// cloud networks are connected to the vSwitch see [`AsyncRobot::get_vswitch`]
#[derive(Debug, Clone, Deserialize)]
pub struct VSwitchReference {
    /// Unique vSwitch ID.
    pub id: VSwitchId,

    /// Name of the vSwitch
    pub name: String,

    /// VLAN ID for the vSwitch.
    ///
    /// VLAN IDs must be in the range 4000..=4091.
    pub vlan: VlanId,

    /// Indicates if the vSwitch has been cancelled or not.
    pub cancelled: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct InternalVSwitch {
    pub id: VSwitchId,
    pub name: String,
    pub vlan: VlanId,
    pub cancelled: bool,
    pub server: Vec<VSwitchServer>,
    pub subnet: Vec<InternalSubnet>,
    pub cloud_network: Vec<InternalCloudNetwork>,
}

impl From<InternalVSwitch> for VSwitch {
    fn from(value: InternalVSwitch) -> Self {
        VSwitch {
            id: value.id,
            name: value.name,
            vlan: value.vlan,
            cancelled: value.cancelled,
            servers: value.server,
            subnets: value.subnet.into_iter().map(IpNet::from).collect(),
            cloud_networks: value
                .cloud_network
                .into_iter()
                .map(CloudNetwork::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct InternalSubnet {
    pub ip: IpAddr,
    pub mask: u8,
}

impl From<InternalSubnet> for IpNet {
    fn from(value: InternalSubnet) -> Self {
        IpNet::new(value.ip, value.mask).unwrap()
    }
}

/// Describes a complete vSwitch configuration.
#[derive(Debug, Clone)]
pub struct VSwitch {
    /// Unique vSwitch ID.
    pub id: VSwitchId,

    /// Name for this vSwitch.
    pub name: String,

    /// VLAN ID associated with traffic over this vSwitch.
    pub vlan: VlanId,

    /// Indicates if the vSwitch has been cancelled.
    pub cancelled: bool,

    /// List of servers connected to this vSwitch.
    pub servers: Vec<VSwitchServer>,

    /// List of subnets associated with this vSwitch.
    pub subnets: Vec<IpNet>,

    /// List of Cloud Networks connected to this vSwitch.
    pub cloud_networks: Vec<CloudNetwork>,
}

/// Indicates the connection status of a server to a vSwitch.
///
/// Connecting or disconnecting a server to/from a vSwitch requires some
/// processing time, and the server won't be immediately available on the vSwitch
/// network.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum ConnectionStatus {
    /// Server is connected and ready.
    #[serde(rename = "ready")]
    Ready,

    /// Server is currently in the process of connecting or disconnecting from the vSwitch.
    #[serde(rename = "in process", alias = "processing")]
    InProcess,

    /// Server connect/disconnect failed.
    #[serde(rename = "failed")]
    Failed,
}

/// Connection status of a server to a vSwitch.
#[derive(Debug, Clone, Deserialize)]
pub struct VSwitchServer {
    /// Server's unique ID.
    #[serde(rename = "server_number")]
    pub id: ServerId,

    /// Status of the server's connection to the vSwitch.
    pub status: ConnectionStatus,
}

#[derive(Debug, Clone, Deserialize)]
struct InternalCloudNetwork {
    pub id: CloudNetworkId,
    pub ip: IpAddr,
    pub mask: u8,
}

impl From<InternalCloudNetwork> for CloudNetwork {
    fn from(value: InternalCloudNetwork) -> Self {
        CloudNetwork {
            id: value.id,
            network: IpNet::new(value.ip, value.mask).unwrap(),
        }
    }
}

/// Identifies a Cloud Network connected to a vSwitch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloudNetwork {
    /// Unique ID for the Cloud Network the vSwitch is connected to.
    pub id: CloudNetworkId,

    /// Subnet of the Cloud Network the vSwitch inhabits.
    pub network: IpNet,
}

/// Cloud Network unique ID.
///
/// Simple wrapper around a u32, to avoid confusion with for example [`VSwitchId`]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CloudNetworkId(pub u32);

impl From<u32> for CloudNetworkId {
    fn from(value: u32) -> Self {
        CloudNetworkId(value)
    }
}

impl From<CloudNetworkId> for u32 {
    fn from(value: CloudNetworkId) -> Self {
        value.0
    }
}

impl Display for CloudNetworkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<u32> for CloudNetworkId {
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        net::{IpAddr, Ipv4Addr},
        str::FromStr,
    };

    use ipnet::{IpNet, Ipv4Net};

    use crate::api::vswitch::{
        CloudNetwork, CloudNetworkId, InternalCloudNetwork, InternalSubnet, VSwitchId, VlanId,
    };

    use super::InternalVSwitch;

    #[test]
    fn deserialize_vswitch() {
        let json = r#"
        {
            "id": 50301,
            "name": "hrobot-test-vswitch-AOLwCPri-re",
            "vlan":4001,
            "cancelled":false,
            "server":[
                {
                    "server_number": 2321379,
                    "server_ip": "138.201.21.47",
                    "server_ipv6_net": "2a01:4f8:171:2c2c::",
                    "status": "processing"
                }
            ],
            "subnet": [],
            "cloud_network": []
        }"#;

        let _ = serde_json::from_str::<InternalVSwitch>(json).unwrap();
    }

    #[test]
    fn vlan_construction() {
        assert_eq!(VlanId::from(4001u16), 4001);

        assert_eq!(VlanId(4001).to_string(), "4001");
    }

    #[test]
    fn vswitch_id_construction() {
        assert_eq!(VSwitchId::from(10101u32), 101010u32);
        assert_eq!(
            VSwitchId::from(101010u32),
            u32::from(VSwitchId::from(10101u32)),
        );
    }

    #[test]
    fn internal_subnet_conversion() {
        assert_eq!(
            IpNet::from(InternalSubnet {
                ip: IpAddr::from_str("127.0.0.0").unwrap(),
                mask: 24
            }),
            IpNet::V4(Ipv4Net::new(Ipv4Addr::new(127, 0, 0, 0), 24).unwrap())
        );
    }

    #[test]
    fn cloud_network_construction() {
        assert_eq!(
            CloudNetwork::from(InternalCloudNetwork {
                id: CloudNetworkId::from(10),
                ip: Ipv4Addr::LOCALHOST.into(),
                mask: 8
            }),
            CloudNetwork {
                id: CloudNetworkId(10),
                network: IpNet::new(Ipv4Addr::LOCALHOST.into(), 8).unwrap()
            }
        );

        assert_eq!(u32::from(CloudNetworkId(10)), 10);
        assert_eq!(CloudNetworkId(10), 10);
    }
}
