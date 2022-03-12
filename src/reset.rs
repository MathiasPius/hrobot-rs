use std::net::{Ipv4Addr, Ipv6Addr};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, OneOrMany};

use crate::{Error, SyncRobot};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ResetOption {
    Power,
    SW,
    HW,
    Man,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Reset {
    #[serde(rename = "server_ip")]
    pub ipv4: Option<Ipv4Addr>,
    #[serde(rename = "server_ipv6_net")]
    pub ipv6_net: Ipv6Addr,
    #[serde(rename = "server_number")]
    pub id: u32,
    #[serde(rename = "type")]
    #[serde_as(deserialize_as = "OneOrMany<_>")]
    pub options: Vec<ResetOption>,
    pub operating_status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ResetResponse {
    pub reset: Reset,
}

impl From<ResetResponse> for Reset {
    fn from(r: ResetResponse) -> Self {
        r.reset
    }
}

pub trait ResetRobot {
    fn list_resets(&self) -> Result<Vec<Reset>, Error>;
    fn get_reset(&self, server_number: u32) -> Result<Reset, Error>;
    fn reset_server(&self, server_number: u32, method: ResetOption) -> Result<Reset, Error>;
}

impl<T> ResetRobot for T
where
    T: SyncRobot,
{
    fn list_resets(&self) -> Result<Vec<Reset>, Error> {
        self.get::<Vec<ResetResponse>>("/reset")
            .map(|r| r.into_iter().map(Reset::from).collect())
    }

    fn get_reset(&self, server_number: u32) -> Result<Reset, Error> {
        self.get::<ResetResponse>(&format!("/reset/{}", server_number))
            .map(Reset::from)
    }

    fn reset_server(&self, server_number: u32, method: ResetOption) -> Result<Reset, Error> {
        #[derive(Serialize)]
        struct ResetServerRequest {
            #[serde(rename = "type")]
            pub method: ResetOption,
        }

        self.post::<ResetResponse, ResetServerRequest>(
            &format!("/reset/{}", server_number),
            ResetServerRequest { method },
        )
        .map(Reset::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Robot;

    #[test]
    #[ignore]
    pub fn list_resets() {
        let robot = Robot::default();

        assert!(robot.list_resets().unwrap().len() > 0);
    }

    #[test]
    #[ignore]
    pub fn get_reset() {
        let robot = Robot::default();

        let resets = robot.list_resets().unwrap();
        assert!(resets.len() > 0);
        assert_eq!(robot.get_reset(resets[0].id).unwrap().ipv4, resets[0].ipv4);
    }

    /*
    /// This was tested once, but it's obviously kinda disruptive...
    #[test]
    #[ignore]
    pub fn reset_server() {
        let robot = Robot::default();

        let resets = robot.list_resets().unwrap();
        robot.reset_server(resets[1].id, ResetOption::HW).unwrap();
    }
    */
}
