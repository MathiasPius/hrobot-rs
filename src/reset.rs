use std::net::{Ipv4Addr, Ipv6Addr};

use serde::{Deserialize, Serialize};

use crate::{APIResult, Error, Robot};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ResetOption {
    Power,
    SW,
    HW,
    Man,
}

#[derive(Debug, Deserialize)]
pub struct Reset {
    pub server_ip: Option<Ipv4Addr>,
    pub server_ipv6_net: Ipv6Addr,
    pub server_number: u32,
    #[serde(rename = "type")]
    pub options: Vec<ResetOption>,
    pub operating_status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ResetResponse {
    pub reset: Reset,
}

pub trait ResetRobot {
    fn list_resets(&self) -> Result<Vec<Reset>, Error>;
    fn get_reset(&self, server_number: u32) -> Result<Reset, Error>;
    fn reset_server(&self, server_number: u32, method: ResetOption) -> Result<Reset, Error>;
}

impl ResetRobot for Robot {
    fn list_resets(&self) -> Result<Vec<Reset>, Error> {
        let result: APIResult<Vec<ResetResponse>> = self.get("/reset")?;

        match result {
            APIResult::Ok(s) => Ok(s.into_iter().map(|s| s.reset).collect()),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn get_reset(&self, server_number: u32) -> Result<Reset, Error> {
        let result: APIResult<ResetResponse> = self.get(&format!("/reset/{}", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.reset),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn reset_server(&self, server_number: u32, method: ResetOption) -> Result<Reset, Error> {
        #[derive(Serialize)]
        struct ResetServerRequest {
            #[serde(rename = "type")]
            pub method: ResetOption,
        }

        let result: APIResult<ResetResponse> = self.post(
            &format!("/reset/{}", server_number),
            ResetServerRequest { method },
        )?;

        match result {
            APIResult::Ok(s) => Ok(s.reset),
            APIResult::Error(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Robot;

    #[test]
    pub fn list_resets() {
        let robot = Robot::default();

        assert!(robot.list_resets().unwrap().len() > 0);
    }

    #[test]
    pub fn get_reset() {
        let robot = Robot::default();

        let resets = robot.list_resets().unwrap();
        assert!(resets.len() > 0);
        assert_eq!(
            robot.get_reset(resets[0].server_number).unwrap().server_ip,
            resets[0].server_ip
        );
    }

    /*
    /// This was tested once, but it's obviously kinda disruptive...
    #[test]
    pub fn reset_server() {
        let robot = Robot::default();

        let resets = robot.list_resets().unwrap();
        robot.reset_server(resets[0].server_number, ResetOption::HW).unwrap();
    }
    */
}
