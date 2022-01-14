use crate::{
    error::{APIResult, Error},
    robot::Robot,
};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug, Deserialize)]
pub enum Status {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "in progress")]
    InProgress,
}

#[derive(Debug, Deserialize)]
pub struct SubnetReference {
    pub ip: IpAddr,
    pub mask: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerFlags {
    pub reset: bool,
    pub rescue: bool,
    pub vnc: bool,
    pub windows: bool,
    pub plesk: bool,
    pub cpanel: bool,
    pub wol: bool,
    pub hot_swap: bool,
    pub linked_storagebox: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub server_ip: Option<Ipv4Addr>,
    pub server_ipv6_net: Ipv6Addr,
    pub server_number: u32,
    pub server_name: String,
    pub product: String,
    pub dc: String,
    pub traffic: String,
    pub status: Status,
    pub cancelled: bool,
    pub paid_until: String,
    #[serde(default)]
    pub ip: Vec<String>,
    #[serde(default)]
    pub subnet: Vec<SubnetReference>,
    #[serde(flatten)]
    pub extended: Option<ServerFlags>,
}

#[derive(Debug, Deserialize)]
struct ServerResponse {
    pub server: Server,
}

#[derive(Debug, Deserialize)]
pub struct Cancellation {
    pub server_ip: Option<Ipv4Addr>,
    pub server_number: u32,
    pub server_name: String,
    pub earliest_cancellation_date: String,
    pub cancelled: bool,
    pub reservation_possible: bool,
    pub reserved: bool,
    pub cancellation_date: Option<String>,
    pub cancellation_reason: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CancellationResponse {
    pub cancellation: Cancellation,
}

pub trait ServerRobot {
    fn list_servers(&self) -> Result<Vec<Server>, Error>;
    fn get_server(&self, id: u32) -> Result<Server, Error>;
    fn rename_server(&self, id: u32, server_name: &str) -> Result<Server, Error>;
    fn get_server_cancellation(&self, id: u32) -> Result<Cancellation, Error>;
}

impl ServerRobot for Robot {
    fn list_servers(&self) -> Result<Vec<Server>, Error> {
        let result: APIResult<Vec<ServerResponse>> = self.get("/server")?;

        match result {
            APIResult::Ok(s) => Ok(s.into_iter().map(|s| s.server).collect()),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn get_server(&self, server_number: u32) -> Result<Server, Error> {
        let result: APIResult<ServerResponse> = self.get(&format!("/server/{}", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.server),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn rename_server(&self, server_number: u32, server_name: &str) -> Result<Server, Error> {
        #[derive(Serialize)]
        struct RenameServerRequest<'a> {
            pub server_name: &'a str,
        }

        let result: APIResult<ServerResponse> = self.post(
            &format!("/server/{}", server_number),
            RenameServerRequest { server_name },
        )?;

        match result {
            APIResult::Ok(s) => Ok(s.server),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn get_server_cancellation(&self, server_number: u32) -> Result<Cancellation, Error> {
        let result: APIResult<CancellationResponse> =
            self.get(&format!("/server/{}/cancellation", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.cancellation),
            APIResult::Error(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Robot;

    #[test]
    pub fn list_servers() {
        let robot = Robot::default();
        println!("{:#?}", robot.list_servers().unwrap());
        assert!(robot.list_servers().unwrap().len() > 0);
    }

    #[test]
    pub fn get_server() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);
        assert_eq!(
            robot
                .get_server(servers[0].server_number)
                .unwrap()
                .server_name,
            servers[0].server_name
        );
    }

    #[test]
    pub fn rename_server() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);

        let old_name = &servers[0].server_name;
        robot
            .rename_server(servers[0].server_number, "test_name")
            .unwrap();

        let new_server = robot.get_server(servers[0].server_number).unwrap();
        assert_eq!(new_server.server_name, "test_name");
        robot
            .rename_server(servers[0].server_number, old_name)
            .unwrap();
    }

    #[test]
    pub fn get_server_cancellation() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);
        let cancellation = robot
            .get_server_cancellation(servers[0].server_number)
            .unwrap();

        assert_eq!(cancellation.server_number, servers[0].server_number);
        if cancellation.cancelled {
            assert!(cancellation.cancellation_date.is_some());
        } else {
            assert!(cancellation.cancellation_date.is_none());
        }
    }
}
