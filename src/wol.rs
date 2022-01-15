use std::net::{Ipv4Addr, Ipv6Addr};

use serde::Deserialize;

use crate::{Error, SyncRobot};

#[derive(Debug, Deserialize)]
pub struct WakeOnLan {
    #[serde(rename = "server_ip")]
    pub ip: Ipv4Addr,
    #[serde(rename = "server_ipv6_net")]
    pub ipv6_net: Ipv6Addr,
    #[serde(rename = "server_number")]
    pub id: u32,
}

#[derive(Debug, Deserialize)]
struct WakeOnLanResponse {
    pub wol: WakeOnLan,
}

impl From<WakeOnLanResponse> for WakeOnLan {
    fn from(w: WakeOnLanResponse) -> Self {
        w.wol
    }
}

pub trait WakeOnLanRobot {
    fn get_wol(&self, server_number: u32) -> Result<WakeOnLan, Error>;
    fn trigger_wol(&self, server_number: u32) -> Result<WakeOnLan, Error>;
}

impl<T> WakeOnLanRobot for T
where
    T: SyncRobot,
{
    fn get_wol(&self, server_number: u32) -> Result<WakeOnLan, Error> {
        self.get::<WakeOnLanResponse>(&format!("/wol/{}", server_number))
            .map(WakeOnLan::from)
    }

    fn trigger_wol(&self, server_number: u32) -> Result<WakeOnLan, Error> {
        self.post::<WakeOnLanResponse, ()>(&format!("/wol/{}", server_number), ())
            .map(WakeOnLan::from)
    }
}
