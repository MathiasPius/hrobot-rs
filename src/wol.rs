use std::net::{Ipv4Addr, Ipv6Addr};

use serde::Deserialize;

use crate::{Error, Robot};

#[derive(Debug, Deserialize)]
pub struct WakeOnLan {
    pub server_ip: Ipv4Addr,
    pub server_ipv6_net: Ipv6Addr,
    pub server_number: u32,
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

impl WakeOnLanRobot for Robot {
    fn get_wol(&self, server_number: u32) -> Result<WakeOnLan, Error> {
        self.get::<WakeOnLanResponse>(&format!("/wol/{}", server_number))
            .map(WakeOnLan::from)
    }

    fn trigger_wol(&self, server_number: u32) -> Result<WakeOnLan, Error> {
        self.post::<WakeOnLanResponse, ()>(&format!("/wol/{}", server_number), ())
            .map(WakeOnLan::from)
    }
}
