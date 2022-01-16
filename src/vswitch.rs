use std::net::{Ipv4Addr, Ipv6Addr};

use serde::{Deserialize, Serialize};

use crate::{Error, SyncRobot};

#[derive(Debug, Deserialize)]
pub struct Vswitch {
    pub id: u32,
    pub name: String,
    pub vlan: u32,
    pub cancelled: bool,
}

#[derive(Debug, Deserialize)]
pub enum VswitchServerStatus {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "in process")]
    InProcess,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Deserialize)]
pub struct VswitchServer {
    #[serde(rename = "server_ip")]
    pub ipv4: Option<Ipv4Addr>,
    #[serde(rename = "server_ipv6_net")]
    pub ipv6_net: Ipv6Addr,
    #[serde(rename = "server_number")]
    pub id: u32,
    pub status: VswitchServerStatus,
}

#[derive(Debug, Deserialize)]
pub struct VswitchSubnet {
    pub ip: Ipv4Addr,
    pub mask: u8,
    pub gateway: Ipv4Addr,
}

#[derive(Debug, Deserialize)]
pub struct VswitchCloudNetwork {
    pub id: u32,
    pub ip: Ipv4Addr,
    pub mask: u8,
    pub gateway: Ipv4Addr,
}

#[derive(Debug, Deserialize)]
pub struct VswitchExtended {
    #[serde(flatten)]
    pub vswitch: Vswitch,
    #[serde(rename = "server")]
    pub servers: Vec<VswitchServer>,
    #[serde(rename = "subnet")]
    pub subnets: Vec<VswitchSubnet>,
    #[serde(rename = "cloud_network")]
    pub cloud_networks: Vec<VswitchCloudNetwork>,
}

pub trait VswitchRobot {
    fn list_vswitches(&self) -> Result<Vec<Vswitch>, Error>;
    fn get_vswitch(&self, id: u32) -> Result<VswitchExtended, Error>;
    fn delete_vswitch(&self, id: u32, cancellation_date: &str) -> Result<(), Error>;
    fn create_vswitch(&self, name: &str, vlan: u32) -> Result<VswitchExtended, Error>;
    fn rename_vswitch(&self, id: u32, name: &str) -> Result<(), Error>;
    fn change_vswitch_vlan(&self, id: u32, vlan: u32) -> Result<(), Error>;
    fn add_server_to_vswitch(&self, vswitch_id: u32, servers: &[u32]) -> Result<(), Error>;
    fn remove_server_from_vswitch(&self, vswitch_id: u32, servers: &[u32]) -> Result<(), Error>;
}

#[derive(Serialize)]
struct VSwitchUpdateRequest<'a> {
    pub name: Option<&'a str>,
    pub vlan: Option<u32>,
}

impl<T> VswitchRobot for T
where
    T: SyncRobot,
{
    fn list_vswitches(&self) -> Result<Vec<Vswitch>, Error> {
        self.get("/vswitch")
    }

    fn get_vswitch(&self, id: u32) -> Result<VswitchExtended, Error> {
        self.get(&format!("/vswitch/{}", id))
    }

    fn delete_vswitch(&self, id: u32, cancellation_date: &str) -> Result<(), Error> {
        #[derive(Serialize)]
        struct VswitchDeleteRequest<'a> {
            pub cancellation_date: &'a str,
        }
        self.delete(
            &format!("/vswitch/{}", id),
            VswitchDeleteRequest { cancellation_date },
        )
        .or_else(|e| {
            if let Error::Decode(ref e) = e {
                if e.is_eof() {
                    return Ok(());
                }
            }
            Err(e)
        })
    }

    fn create_vswitch(&self, name: &str, vlan: u32) -> Result<VswitchExtended, Error> {
        self.post(
            "/vswitch",
            VSwitchUpdateRequest {
                name: Some(name),
                vlan: Some(vlan),
            },
        )
    }

    fn rename_vswitch(&self, id: u32, name: &str) -> Result<(), Error> {
        self.post(
            &format!("/vswitch/{}", id),
            VSwitchUpdateRequest {
                name: Some(name),
                vlan: None,
            },
        )
    }

    fn change_vswitch_vlan(&self, id: u32, vlan: u32) -> Result<(), Error> {
        self.post(
            &format!("/vswitch/{}", id),
            VSwitchUpdateRequest {
                name: None,
                vlan: Some(vlan),
            },
        )
    }

    fn add_server_to_vswitch(&self, vswitch_id: u32, server_ids: &[u32]) -> Result<(), Error> {
        self.post_raw(
            &format!("/vswitch/{}/server", vswitch_id),
            server_ids
                .iter()
                .map(|id| format!("server[]={}", id))
                .collect::<Vec<_>>()
                .join("&"),
        )
        // This endpoint returns nothing (empty string) on success
        // so we need to cast that to the unit
        .or_else(|e| {
            if let Error::Decode(ref e) = e {
                if e.is_eof() {
                    return Ok(());
                }
            }
            Err(e)
        })
    }

    fn remove_server_from_vswitch(&self, vswitch_id: u32, server_ids: &[u32]) -> Result<(), Error> {
        self.delete_raw(
            &format!("/vswitch/{}/server", vswitch_id),
            server_ids
                .iter()
                .map(|id| format!("server[]={}", id))
                .collect::<Vec<_>>()
                .join("&"),
        )
        // This endpoint returns nothing (empty string) on success
        // so we need to cast that to the unit
        .or_else(|e| {
            if let Error::Decode(ref e) = e {
                if e.is_eof() {
                    return Ok(());
                }
            }
            Err(e)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::VswitchRobot;
    use crate::Robot;

    #[test]
    #[ignore]
    fn list_vswitches() {
        let robot = Robot::default();

        let vswitches = robot.list_vswitches().unwrap();
        assert!(vswitches.len() > 0);
    }

    #[test]
    #[ignore]
    fn get_vswitch() {
        let robot = Robot::default();

        let vswitches = robot.list_vswitches().unwrap();
        assert!(vswitches.len() > 0);

        let vswitch = robot.get_vswitch(vswitches[0].id).unwrap();
        println!("{:#?}", vswitch);
    }

    /*
    This test was used to do a full cycle test of the vSwitch API, but it takes forever,
    is not very intelligent, and creates unnecessary vSwitches with Hetzner which then
    have to cleaned up afterwards.

    #[test]
    #[ignore]
    fn vswitch_full_cycle() {
        use crate::{APIError, Error, ServerRobot};
        let robot = Robot::default();

        let vswitch = robot.create_vswitch("test-vswitch", 4083).unwrap();
        println!("vSwitch created");

        let servers = robot.list_servers().unwrap();

        for _ in 0..10 {
            // Retry 10 times (sleeping 5 seconds inbetween)
            if robot
                .add_server_to_vswitch(vswitch.vswitch.id, &[servers[0].id])
                .map(Option::from)
                .or_else(|e| {
                    if matches!(e, Error::API(APIError::VswitchInProcess { .. })) {
                        Ok(None)
                    } else {
                        Err(e)
                    }
                })
                .unwrap()
                .is_some()
            {
                println!("Success!");
                break;
            }

            println!("Sleeping for 5 seconds");
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
        println!("Server added, removing it again.");
        std::thread::sleep(std::time::Duration::from_secs(30));

        for _ in 0..10 {
            // Retry 10 times (sleeping 5 seconds inbetween)
            if robot
                .remove_server_from_vswitch(vswitch.vswitch.id, &[servers[0].id])
                .map(Option::from)
                .or_else(|e| {
                    if matches!(e, Error::API(APIError::VswitchInProcess { .. })) {
                        Ok(None)
                    } else {
                        Err(e)
                    }
                })
                .unwrap()
                .is_some()
            {
                println!("Success!");
                break;
            }
            println!("Sleeping for 5 seconds");
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
        println!("Server removed. Deleting vSwitch");
        std::thread::sleep(std::time::Duration::from_secs(30));

        for _ in 0..10 {
            // Retry 10 times (sleeping 5 seconds inbetween)
            if robot
                .delete_vswitch(vswitch.vswitch.id, "2022-01-16")
                .map(Option::from)
                .or_else(|e| {
                    if matches!(e, Error::API(APIError::VswitchInProcess { .. })) {
                        Ok(None)
                    } else {
                        Err(e)
                    }
                })
                .unwrap()
                .is_some()
            {
                println!("Success!");
                break;
            }
            println!("Sleeping for 5 seconds");
            std::thread::sleep(std::time::Duration::from_secs(5));
        }

        println!("vSwitch full cycle test complete");
    }
    */
}
