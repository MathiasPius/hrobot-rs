use crate::{
    error::{APIResult, Error},
    robot::Robot,
};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Debug, Deserialize, Serialize)]
pub struct TrafficWarnings {
    pub traffic_warnings: bool,
    pub traffic_hourly: u32,
    pub traffic_daily: u32,
    pub traffic_monthly: u32,
}

#[derive(Debug, Deserialize)]
pub struct Ip {
    pub ip: Ipv4Addr,
    pub server_ip: Ipv4Addr,
    pub server_number: u32,
    pub locked: bool,
    pub separate_mac: Option<String>,
    #[serde(flatten)]
    pub traffic_warnings: TrafficWarnings,
}

#[derive(Debug, Deserialize)]
pub struct IpResponse {
    pub ip: Ip,
}

#[derive(Debug, Deserialize)]
pub struct Mac {
    pub ip: Ipv4Addr,
    pub mac: String,
}

#[derive(Debug, Deserialize)]
pub struct MacResponse {
    pub mac: Mac,
}

pub trait IpRobot {
    fn list_ips(&self) -> Result<Vec<Ip>, Error>;
    fn get_ip(&self, ip: Ipv4Addr) -> Result<Ip, Error>;
    fn get_mac(&self, ip: Ipv4Addr) -> Result<Mac, Error>;
}

impl IpRobot for Robot {
    fn list_ips(&self) -> Result<Vec<Ip>, Error> {
        let result: APIResult<Vec<IpResponse>> = self.get("/ip")?;

        match result {
            APIResult::Ok(s) => Ok(s.into_iter().map(|s| s.ip).collect()),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn get_ip(&self, ip: Ipv4Addr) -> Result<Ip, Error> {
        let result: APIResult<IpResponse> = self.get(&format!("/ip/{}", ip))?;

        match result {
            APIResult::Ok(s) => Ok(s.ip),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn get_mac(&self, ip: Ipv4Addr) -> Result<Mac, Error> {
        let result: APIResult<MacResponse> = self.get(&format!("/ip/{}/mac", ip))?;

        match result {
            APIResult::Ok(s) => Ok(s.mac),
            APIResult::Error(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IpRobot;
    use crate::robot::Robot;

    #[test]
    pub fn list_ips() {
        let robot = Robot::default();

        assert!(robot.list_ips().unwrap().len() > 0);
    }

    #[test]
    pub fn get_server() {
        let robot = Robot::default();

        let ips = robot.list_ips().unwrap();
        assert!(ips.len() > 0);
        assert_eq!(robot.get_ip(ips[0].ip).unwrap().ip, ips[0].ip);
    }
}
