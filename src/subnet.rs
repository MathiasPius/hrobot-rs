use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr};

use crate::{error::Error, ip::TrafficWarnings, SyncRobot};

#[derive(Debug, Deserialize)]
pub struct Subnet {
    pub ip: IpAddr,
    pub mask: u8,
    pub gateway: IpAddr,
    pub server_ip: Option<Ipv4Addr>,
    #[serde(rename = "server_number")]
    pub id: u32,
    pub failover: bool,
    pub locked: bool,
    pub vswitch_id: Option<u32>,
    #[serde(flatten)]
    pub traffic_warnings: TrafficWarnings,
}

#[derive(Debug, Deserialize)]
struct SubnetResponse {
    subnet: Subnet,
}

impl From<SubnetResponse> for Subnet {
    fn from(s: SubnetResponse) -> Self {
        s.subnet
    }
}

pub trait SubnetRobot {
    fn list_subnets(&self) -> Result<Vec<Subnet>, Error>;
    fn get_subnet(&self, subnet: IpAddr) -> Result<Subnet, Error>;
}

impl<T> SubnetRobot for T
where
    T: SyncRobot,
{
    fn list_subnets(&self) -> Result<Vec<Subnet>, Error> {
        self.get::<Vec<SubnetResponse>>("/subnet")
            .map(|s| s.into_iter().map(Subnet::from).collect())
    }

    fn get_subnet(&self, subnet: IpAddr) -> Result<Subnet, Error> {
        self.get::<SubnetResponse>(&format!("/subnet/{}", subnet))
            .map(Subnet::from)
    }
}

#[cfg(test)]
mod tests {
    use super::SubnetRobot;
    use crate::Robot;

    #[test]
    #[ignore]
    fn list_subnets() {
        let robot = Robot::default();

        let subnets = robot.list_subnets().unwrap();
        assert!(subnets.len() > 0);
    }

    #[test]
    #[ignore]
    fn get_subnet() {
        let robot = Robot::default();

        let subnets = robot.list_subnets().unwrap();
        assert!(subnets.len() > 0);
        assert_eq!(robot.get_subnet(subnets[0].ip).unwrap().ip, subnets[0].ip);
    }
}
