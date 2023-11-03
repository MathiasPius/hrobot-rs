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

fn delete_vswitch(vswitch_id: VSwitchId, date: Date) -> UnauthenticatedRequest<Empty> {
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
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::vswitch::VSwitchId;
    /// # use hrobot::time::{Date, Month};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.cancel_vswitch(
    ///     VSwitchId(124567),
    ///     Date::from_calendar_date(2023, Month::July, 10).unwrap()
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn cancel_vswitch(
        &self,
        vswitch_id: VSwitchId,
        cancellation_date: Date,
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
#[derive(Debug, Clone, Deserialize)]
pub enum ConnectionStatus {
    /// Server is connected and ready.
    #[serde(rename = "ready")]
    Ready,

    /// Server is currently in the process of connecting or disconnecting from the vSwitch.
    #[serde(rename = "in process")]
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
#[derive(Debug, Clone)]
pub struct CloudNetwork {
    /// Unique ID for the Cloud Network the vSwitch is connected to.
    pub id: CloudNetworkId,

    /// Subnet of the Cloud Network the vSwitch inhabits.
    pub network: IpNet,
}

/// Cloud Netowrk unique ID.
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
    #[cfg(feature = "non-disruptive-tests")]
    mod non_disruptive_tests {
        use serial_test::serial;
        use tracing::info;
        use tracing_test::traced_test;

        #[tokio::test]
        #[traced_test]
        #[serial("vswitch")]
        async fn test_list_vswitches() {
            let _ = dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let vswitches = robot.list_vswitches().await.unwrap();
            info!("{vswitches:#?}");
        }

        #[tokio::test]
        #[traced_test]
        #[serial("vswitch")]
        async fn test_get_vswitch() {
            let _ = dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let vswitches = robot.list_vswitches().await.unwrap();
            info!("{vswitches:#?}");

            if let Some(vswitch) = vswitches.first() {
                let vswitch = robot.get_vswitch(vswitch.id).await.unwrap();
                info!("{vswitch:#?}");
            }
        }
    }

    #[cfg(feature = "disruptive-tests")]
    mod disruptive_tests {
        use serial_test::serial;
        use time::OffsetDateTime;
        use tracing_test::traced_test;

        use crate::api::{server::ServerId, vswitch::VlanId};

        #[tokio::test]
        #[traced_test]
        #[ignore = "modifies vswitch connectivity of servers"]
        #[serial("vswitch")]
        async fn test_vswitch_end_to_end() {
            let _ = dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let vswitch = robot
                .create_vswitch("hrobot-test-vswitch-1", VlanId(4076))
                .await
                .unwrap();

            // Rename and change the VLAN ID.
            robot
                .update_vswitch(vswitch.id, "hrobot-test-vswitch-2", VlanId(4077))
                .await
                .unwrap();

            tokio::time::sleep(std::time::Duration::from_secs(120)).await;
            let vswitch = robot.get_vswitch(vswitch.id).await.unwrap();

            assert_eq!(vswitch.name, "hrobot-test-vswitch-2");
            assert_eq!(vswitch.vlan, VlanId(4077));

            assert!(vswitch.subnets.is_empty());
            assert!(vswitch.servers.is_empty());
            assert!(vswitch.cloud_networks.is_empty());

            if let Some(server) = robot.list_servers().await.unwrap().first() {
                // Attempt to connect the server to the vswitch.
                robot
                    .connect_vswitch_servers(vswitch.id, &[server.id])
                    .await
                    .unwrap();
                tokio::time::sleep(std::time::Duration::from_secs(120)).await;

                // Verify that the server is connected.
                let connected_vswitch = robot.get_vswitch(vswitch.id).await.unwrap();

                assert_eq!(connected_vswitch.servers.len(), 1);
                assert_eq!(connected_vswitch.servers[0].id, server.id);

                // Disconnect the server again.
                robot
                    .disconnect_vswitch_servers(vswitch.id, &[server.id])
                    .await
                    .unwrap();

                tokio::time::sleep(std::time::Duration::from_secs(120)).await;

                let disconnected_vswitch = robot.get_vswitch(vswitch.id).await.unwrap();

                assert!(disconnected_vswitch.servers.is_empty());
            }

            robot
                .cancel_vswitch(vswitch.id, OffsetDateTime::now_utc().date())
                .await
                .unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(120)).await;
        }

        #[tokio::test]
        #[traced_test]
        #[ignore = "modifies vswitch connectivity of servers"]
        #[serial("vswitch")]
        async fn test_connect_disconnect_multiple() {
            let _ = dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let vswitch = robot
                .create_vswitch("hrobot-test-vswitch-10", VlanId(4073))
                .await
                .unwrap();

            // Connect all servers
            let servers: Vec<ServerId> = robot
                .list_servers()
                .await
                .unwrap()
                .into_iter()
                .map(|server| server.id)
                .collect();

            robot
                .connect_vswitch_servers(vswitch.id, &servers)
                .await
                .unwrap();

            tokio::time::sleep(std::time::Duration::from_secs(120)).await;
            robot
                .disconnect_vswitch_servers(vswitch.id, &servers)
                .await
                .unwrap();

            robot
                .cancel_vswitch(vswitch.id, OffsetDateTime::now_utc().date())
                .await
                .unwrap();

            tokio::time::sleep(std::time::Duration::from_secs(120)).await;
        }
    }
}
