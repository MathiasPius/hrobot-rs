use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr};

use crate::{
    error::{APIResult, Error},
    ip::TrafficWarnings,
    robot::Robot,
};

#[derive(Debug, Deserialize)]
pub struct Subnet {
    pub ip: IpAddr,
    pub mask: u8,
    pub gateway: IpAddr,
    pub server_ip: Option<Ipv4Addr>,
    pub server_number: u32,
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

pub trait SubnetRobot {
    fn list_subnets(&self) -> Result<Vec<Subnet>, Error>;
    fn get_subnet(&self, subnet: IpAddr) -> Result<Subnet, Error>;
}

impl SubnetRobot for Robot {
    fn list_subnets(&self) -> Result<Vec<Subnet>, Error> {
        let result: APIResult<Vec<SubnetResponse>> = self.get("/subnet")?;

        match result {
            APIResult::Ok(s) => Ok(s.into_iter().map(|s| s.subnet).collect()),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn get_subnet(&self, subnet: IpAddr) -> Result<Subnet, Error> {
        let result: APIResult<SubnetResponse> = self.get(&format!("/subnet/{}", subnet))?;

        match result {
            APIResult::Ok(s) => Ok(s.subnet),
            APIResult::Error(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SubnetRobot;
    use crate::Robot;

    #[test]
    fn list_subnets() {
        let robot = Robot::default();

        let subnets = robot.list_subnets().unwrap();
        assert!(subnets.len() > 0);
    }

    #[test]
    fn get_subnet() {
        let robot = Robot::default();

        let subnets = robot.list_subnets().unwrap();
        assert!(subnets.len() > 0);
        assert_eq!(robot.get_subnet(subnets[0].ip).unwrap().ip, subnets[0].ip);
    }
}
