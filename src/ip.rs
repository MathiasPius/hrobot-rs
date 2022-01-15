use crate::{error::Error, SyncRobot};
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
struct IpResponse {
    pub ip: Ip,
}

impl From<IpResponse> for Ip {
    fn from(i: IpResponse) -> Self {
        i.ip
    }
}

#[derive(Debug, Deserialize)]
pub struct Mac {
    pub ip: Ipv4Addr,
    pub mac: String,
}

#[derive(Debug, Deserialize)]
struct MacResponse {
    pub mac: Mac,
}

impl From<MacResponse> for Mac {
    fn from(m: MacResponse) -> Self {
        m.mac
    }
}

pub trait IpRobot {
    fn list_ips(&self) -> Result<Vec<Ip>, Error>;
    fn get_ip(&self, ip: Ipv4Addr) -> Result<Ip, Error>;
    fn get_mac(&self, ip: Ipv4Addr) -> Result<Mac, Error>;
}

impl<T> IpRobot for T
where
    T: SyncRobot,
{
    fn list_ips(&self) -> Result<Vec<Ip>, Error> {
        self.get::<Vec<IpResponse>>("/ip")
            .map(|i| i.into_iter().map(Ip::from).collect())
    }

    fn get_ip(&self, ip: Ipv4Addr) -> Result<Ip, Error> {
        self.get::<IpResponse>(&format!("/ip/{}", ip)).map(Ip::from)
    }

    fn get_mac(&self, ip: Ipv4Addr) -> Result<Mac, Error> {
        self.get::<MacResponse>(&format!("/ip/{}/mac", ip))
            .map(Mac::from)
    }
}

#[cfg(test)]
mod tests {
    use super::IpRobot;
    use crate::robot::Robot;

    #[test]
    #[ignore]
    pub fn list_ips() {
        let robot = Robot::default();

        assert!(robot.list_ips().unwrap().len() > 0);
    }

    #[test]
    #[ignore]
    pub fn get_server() {
        let robot = Robot::default();

        let ips = robot.list_ips().unwrap();
        assert!(ips.len() > 0);
        assert_eq!(robot.get_ip(ips[0].ip).unwrap().ip, ips[0].ip);
    }
}
