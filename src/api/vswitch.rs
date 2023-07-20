use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{AsyncHttpClient, AsyncRobot, error::Error};

use super::{UnauthenticatedRequest, wrapper::{List, Single}};

fn list_vswitches() -> UnauthenticatedRequest<List<VSwitch>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/vswitch")
}

fn get_vswitch(vswitch: VSwitchId) -> UnauthenticatedRequest<Single<VSwitch>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/vswitch/{vswitch}"))    
}


impl<Client: AsyncHttpClient> AsyncRobot<Client> {
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
    pub async fn list_vswitches(&self) -> Result<Vec<VSwitch>, Error> {
        Ok(self.go(list_vswitches()).await?.0)
    }
    
    /// Get vSwitch information.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_vswitch(VSwitchId(123456)).await.unwrap();
    /// # }
    /// ```
    pub async fn get_vswitch(&self, vswitch: VSwitchId) -> Result<VSwitch, Error> {
        Ok(self.go(get_vswitch(vswitch)).await?.0)
    }
}


/// vSwitch unique ID.
///
/// Simple wrapper around a u32, to avoid confusion with for example [`VlanId`](crate::api::vswitch::VlanId)
/// and to make it intuitive what kind of argument you need to give to functions like [`AsyncRobot::get_vswitch`](crate::AsyncRobot::get_vswitch()).
///
/// Using a plain integer means it isn't clear what the argument is, is it a counter of my vSwitches, where the argument
/// is in range `0..N` where `N` is the number of vswitches in my account, or is it a limiter, like get first `N`
/// vswitches, for example.
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


/// VLAN ID.
///
/// Simple wrapper around a u16, to avoid confusion with vSwitch ID.
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

#[derive(Debug, Clone, Deserialize)]
pub struct VSwitch {
    pub id: VSwitchId,
    pub name: String,
    pub vlan: VlanId,
    pub cancelled: bool,
}