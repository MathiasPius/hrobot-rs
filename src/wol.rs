use std::net::{Ipv4Addr, Ipv6Addr};

use serde::Deserialize;

use crate::{APIResult, Error, Robot};

#[derive(Debug, Deserialize)]
pub struct WakeOnLan {
    pub server_ip: Ipv4Addr,
    pub server_ipv6_net: Ipv6Addr,
    pub server_number: u32,
}

#[derive(Debug, Deserialize)]
pub struct WakeOnLanResponse {
    pub wol: WakeOnLan,
}

pub trait WakeOnLanRobot {
    fn get_wol(&self, server_number: u32) -> Result<WakeOnLan, Error>;
    fn trigger_wol(&self, server_number: u32) -> Result<WakeOnLan, Error>;
}

impl WakeOnLanRobot for Robot {
    fn get_wol(&self, server_number: u32) -> Result<WakeOnLan, Error> {
        let result: APIResult<WakeOnLanResponse> = self.get(&format!("/wol/{}", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.wol),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn trigger_wol(&self, server_number: u32) -> Result<WakeOnLan, Error> {
        let result: APIResult<WakeOnLanResponse> =
            self.post(&format!("/wol/{}", server_number), ())?;

        match result {
            APIResult::Ok(s) => Ok(s.wol),
            APIResult::Error(e) => Err(e.into()),
        }
    }
}
