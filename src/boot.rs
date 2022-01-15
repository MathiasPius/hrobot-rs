use std::net::{Ipv4Addr, Ipv6Addr};

use serde::{Deserialize, Serialize};

use crate::{APIResult, Error, Robot};

#[derive(Debug, Deserialize)]
pub struct RescueConfiguration {
    pub server_ip: Option<Ipv4Addr>,
    pub server_ipv6_net: Ipv6Addr,
    pub server_number: u32,
    #[serde(deserialize_with = "crate::string_or_seq_string")]
    pub os: Vec<String>,
    #[serde(deserialize_with = "crate::num_or_seq_num")]
    pub arch: Vec<u64>,
    pub active: bool,
    pub password: Option<String>,
    pub authorized_key: Vec<String>,
    pub host_key: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct LinuxConfiguration {
    pub server_ip: Option<Ipv4Addr>,
    pub server_ipv6_net: Ipv6Addr,
    pub server_number: u32,
    #[serde(deserialize_with = "crate::string_or_seq_string")]
    pub dist: Vec<String>,
    #[serde(deserialize_with = "crate::num_or_seq_num")]
    pub arch: Vec<u64>,
    #[serde(deserialize_with = "crate::string_or_seq_string")]
    pub lang: Vec<String>,
    pub active: bool,
    pub password: Option<String>,
    pub authorized_key: Vec<String>,
    pub host_key: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct VncConfiguration {
    pub server_ip: Option<Ipv4Addr>,
    pub server_ipv6_net: Ipv6Addr,
    pub server_number: u32,
    #[serde(deserialize_with = "crate::string_or_seq_string")]
    pub dist: Vec<String>,
    #[serde(deserialize_with = "crate::num_or_seq_num")]
    pub arch: Vec<u64>,
    #[serde(deserialize_with = "crate::string_or_seq_string")]
    pub lang: Vec<String>,
    pub active: bool,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WindowsConfiguration {
    pub server_ip: Option<Ipv4Addr>,
    pub server_ipv6_net: Ipv6Addr,
    pub server_number: u32,
    #[serde(deserialize_with = "crate::string_or_seq_string")]
    pub dist: Vec<String>,
    #[serde(deserialize_with = "crate::num_or_seq_num")]
    pub arch: Vec<u64>,
    #[serde(deserialize_with = "crate::string_or_seq_string")]
    pub lang: Vec<String>,
    pub active: bool,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PleskConfiguration {
    pub server_ip: Option<Ipv4Addr>,
    pub server_ipv6_net: Ipv6Addr,
    pub server_number: u32,
    #[serde(deserialize_with = "crate::string_or_seq_string")]
    pub dist: Vec<String>,
    #[serde(deserialize_with = "crate::num_or_seq_num")]
    pub arch: Vec<u64>,
    #[serde(deserialize_with = "crate::string_or_seq_string")]
    pub lang: Vec<String>,
    pub active: bool,
    pub password: Option<String>,
    pub hostname: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CPanelConfiguration {
    pub server_ip: Option<Ipv4Addr>,
    pub server_ipv6_net: Ipv6Addr,
    pub server_number: u32,
    #[serde(deserialize_with = "crate::string_or_seq_string")]
    pub dist: Vec<String>,
    #[serde(deserialize_with = "crate::num_or_seq_num")]
    pub arch: Vec<u64>,
    #[serde(deserialize_with = "crate::string_or_seq_string")]
    pub lang: Vec<String>,
    pub active: bool,
    pub password: Option<String>,
    pub hostname: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BootConfiguration {
    pub rescue: Option<RescueConfiguration>,
    pub linux: Option<LinuxConfiguration>,
    pub vnc: Option<VncConfiguration>,
    pub windows: Option<WindowsConfiguration>,
    pub plesk: Option<PleskConfiguration>,
    pub cpanel: Option<CPanelConfiguration>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActiveBootConfiguration {
    Rescue(RescueConfiguration),
    Linux(LinuxConfiguration),
    Vnc(VncConfiguration),
    Windows(WindowsConfiguration),
    Plesk(PleskConfiguration),
    CPanel(CPanelConfiguration),
}

impl From<BootConfiguration> for Option<ActiveBootConfiguration> {
    fn from(value: BootConfiguration) -> Self {
        if let Some(c) = value.rescue {
            if c.active {
                return Some(ActiveBootConfiguration::Rescue(c));
            }
        }

        if let Some(c) = value.linux {
            if c.active {
                return Some(ActiveBootConfiguration::Linux(c));
            }
        }

        if let Some(c) = value.vnc {
            if c.active {
                return Some(ActiveBootConfiguration::Vnc(c));
            }
        }

        if let Some(c) = value.windows {
            if c.active {
                return Some(ActiveBootConfiguration::Windows(c));
            }
        }

        if let Some(c) = value.plesk {
            if c.active {
                return Some(ActiveBootConfiguration::Plesk(c));
            }
        }

        if let Some(c) = value.cpanel {
            if c.active {
                return Some(ActiveBootConfiguration::CPanel(c));
            }
        }

        None
    }
}

#[derive(Debug, Deserialize)]
struct BootConfigurationResponse {
    pub boot: BootConfiguration,
}

pub trait BootRobot {
    fn list_server_boot_configurations(
        &self,
        server_number: u32,
    ) -> Result<BootConfiguration, Error>;
    fn get_server_active_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<ActiveBootConfiguration>, Error>;

    fn get_server_rescue_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<RescueConfiguration>, Error>;
    fn delete_server_rescue_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<RescueConfiguration>, Error>;

    fn set_server_rescue_boot_configuration(
        &self,
        server_number: u32,
        os: &str,
        arch: Option<u64>,
        authorized_keys: &[&str],
    ) -> Result<Option<RescueConfiguration>, Error>;

    fn get_server_linux_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<LinuxConfiguration>, Error>;

    fn delete_server_linux_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<LinuxConfiguration>, Error>;

    fn set_server_linux_boot_configuration(
        &self,
        server_number: u32,
        dist: &str,
        arch: Option<u64>,
        lang: &str,
        authorized_keys: &[&str],
    ) -> Result<Option<LinuxConfiguration>, Error>;

    fn get_server_vnc_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<VncConfiguration>, Error>;

    fn delete_server_vnc_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<VncConfiguration>, Error>;

    fn set_server_vnc_boot_configuration(
        &self,
        server_number: u32,
        dist: &str,
        arch: Option<u64>,
        lang: &str,
    ) -> Result<Option<VncConfiguration>, Error>;

    fn get_server_windows_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<WindowsConfiguration>, Error>;

    fn delete_server_windows_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<WindowsConfiguration>, Error>;

    fn set_server_windows_boot_configuration(
        &self,
        server_number: u32,
        lang: &str,
    ) -> Result<Option<WindowsConfiguration>, Error>;

    fn get_server_plesk_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<PleskConfiguration>, Error>;

    fn delete_server_plesk_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<PleskConfiguration>, Error>;

    fn set_server_plesk_boot_configuration(
        &self,
        server_number: u32,
        dist: &str,
        arch: Option<u64>,
        lang: &str,
        hostname: &str,
    ) -> Result<Option<PleskConfiguration>, Error>;

    fn get_server_cpanel_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<CPanelConfiguration>, Error>;

    fn delete_server_cpanel_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<CPanelConfiguration>, Error>;

    fn set_server_cpanel_boot_configuration(
        &self,
        server_number: u32,
        dist: &str,
        arch: Option<u64>,
        lang: &str,
        hostname: &str,
    ) -> Result<Option<CPanelConfiguration>, Error>;
}

impl BootRobot for Robot {
    fn list_server_boot_configurations(
        &self,
        server_number: u32,
    ) -> Result<BootConfiguration, Error> {
        let result: APIResult<BootConfigurationResponse> =
            self.get(&format!("/boot/{}", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.boot),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn get_server_active_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<ActiveBootConfiguration>, Error> {
        Ok(self.list_server_boot_configurations(server_number)?.into())
    }

    fn get_server_rescue_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<RescueConfiguration>, Error> {
        let result: APIResult<BootConfiguration> =
            self.get(&format!("/boot/{}/rescue", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.rescue),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn delete_server_rescue_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<RescueConfiguration>, Error> {
        let result: APIResult<BootConfiguration> =
            self.delete(&format!("/boot/{}/rescue", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.rescue),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn set_server_rescue_boot_configuration(
        &self,
        server_number: u32,
        os: &str,
        arch: Option<u64>,
        authorized_keys: &[&str],
    ) -> Result<Option<RescueConfiguration>, Error> {
        #[derive(Serialize)]
        struct SetRescueConfigurationRequest<'a> {
            pub os: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub arch: Option<u64>,
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            pub authorized_key: &'a [&'a str],
        }

        let result: APIResult<BootConfiguration> = self.post(
            &format!("/boot/{}/rescue", server_number),
            SetRescueConfigurationRequest {
                os,
                arch,
                authorized_key: authorized_keys,
            },
        )?;

        match result {
            APIResult::Ok(s) => Ok(s.rescue),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn get_server_linux_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<LinuxConfiguration>, Error> {
        let result: APIResult<BootConfiguration> =
            self.get(&format!("/boot/{}/linux", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.linux),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn delete_server_linux_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<LinuxConfiguration>, Error> {
        let result: APIResult<BootConfiguration> =
            self.delete(&format!("/boot/{}/linux", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.linux),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn set_server_linux_boot_configuration(
        &self,
        server_number: u32,
        dist: &str,
        arch: Option<u64>,
        lang: &str,
        authorized_keys: &[&str],
    ) -> Result<Option<LinuxConfiguration>, Error> {
        #[derive(Serialize)]
        struct SetLinuxConfigurationRequest<'a> {
            pub dist: &'a str,
            pub lang: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub arch: Option<u64>,
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            pub authorized_key: &'a [&'a str],
        }

        let result: APIResult<BootConfiguration> = self.post(
            &format!("/boot/{}/linux", server_number),
            SetLinuxConfigurationRequest {
                dist,
                lang,
                arch,
                authorized_key: authorized_keys,
            },
        )?;

        match result {
            APIResult::Ok(s) => Ok(s.linux),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn get_server_vnc_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<VncConfiguration>, Error> {
        let result: APIResult<BootConfiguration> =
            self.get(&format!("/boot/{}/vnc", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.vnc),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn delete_server_vnc_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<VncConfiguration>, Error> {
        let result: APIResult<BootConfiguration> =
            self.delete(&format!("/boot/{}/vnc", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.vnc),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn set_server_vnc_boot_configuration(
        &self,
        server_number: u32,
        dist: &str,
        arch: Option<u64>,
        lang: &str,
    ) -> Result<Option<VncConfiguration>, Error> {
        #[derive(Serialize)]
        struct SetVncConfigurationRequest<'a> {
            pub dist: &'a str,
            pub lang: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub arch: Option<u64>,
        }

        let result: APIResult<BootConfiguration> = self.post(
            &format!("/boot/{}/vnc", server_number),
            SetVncConfigurationRequest { dist, lang, arch },
        )?;

        match result {
            APIResult::Ok(s) => Ok(s.vnc),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn get_server_windows_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<WindowsConfiguration>, Error> {
        let result: APIResult<BootConfiguration> =
            self.get(&format!("/boot/{}/windows", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.windows),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn delete_server_windows_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<WindowsConfiguration>, Error> {
        let result: APIResult<BootConfiguration> =
            self.delete(&format!("/boot/{}/windows", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.windows),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn set_server_windows_boot_configuration(
        &self,
        server_number: u32,
        lang: &str,
    ) -> Result<Option<WindowsConfiguration>, Error> {
        #[derive(Serialize)]
        struct SetWindowsConfigurationRequest<'a> {
            pub lang: &'a str,
        }

        let result: APIResult<BootConfiguration> = self.post(
            &format!("/boot/{}/windows", server_number),
            SetWindowsConfigurationRequest { lang },
        )?;

        match result {
            APIResult::Ok(s) => Ok(s.windows),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn get_server_plesk_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<PleskConfiguration>, Error> {
        let result: APIResult<BootConfiguration> =
            self.get(&format!("/boot/{}/plesk", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.plesk),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn delete_server_plesk_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<PleskConfiguration>, Error> {
        let result: APIResult<BootConfiguration> =
            self.delete(&format!("/boot/{}/plesk", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.plesk),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn set_server_plesk_boot_configuration(
        &self,
        server_number: u32,
        dist: &str,
        arch: Option<u64>,
        lang: &str,
        hostname: &str,
    ) -> Result<Option<PleskConfiguration>, Error> {
        #[derive(Serialize)]
        struct SetPleskConfigurationRequest<'a> {
            pub dist: &'a str,
            pub lang: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub arch: Option<u64>,
            pub hostname: &'a str,
        }

        let result: APIResult<BootConfiguration> = self.post(
            &format!("/boot/{}/plesk", server_number),
            SetPleskConfigurationRequest {
                dist,
                lang,
                arch,
                hostname,
            },
        )?;

        match result {
            APIResult::Ok(s) => Ok(s.plesk),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn get_server_cpanel_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<CPanelConfiguration>, Error> {
        let result: APIResult<BootConfiguration> =
            self.get(&format!("/boot/{}/cpanel", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.cpanel),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn delete_server_cpanel_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<CPanelConfiguration>, Error> {
        let result: APIResult<BootConfiguration> =
            self.delete(&format!("/boot/{}/cpanel", server_number))?;

        match result {
            APIResult::Ok(s) => Ok(s.cpanel),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn set_server_cpanel_boot_configuration(
        &self,
        server_number: u32,
        dist: &str,
        arch: Option<u64>,
        lang: &str,
        hostname: &str,
    ) -> Result<Option<CPanelConfiguration>, Error> {
        #[derive(Serialize)]
        struct SetCpanelConfigurationRequest<'a> {
            pub dist: &'a str,
            pub lang: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub arch: Option<u64>,
            pub hostname: &'a str,
        }

        let result: APIResult<BootConfiguration> = self.post(
            &format!("/boot/{}/cpanel", server_number),
            SetCpanelConfigurationRequest {
                dist,
                lang,
                arch,
                hostname,
            },
        )?;

        match result {
            APIResult::Ok(s) => Ok(s.cpanel),
            APIResult::Error(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Robot, ServerRobot};
    use serial_test::serial;

    #[test]
    fn list_server_boot_configuration() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);

        let configs = robot.list_server_boot_configurations(servers[0].server_number);

        assert!(configs.is_ok());
        println!("{:#?}", configs.unwrap());
    }

    #[test]
    fn get_server_rescue_boot_configuration() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);

        let config = robot
            .get_server_rescue_boot_configuration(servers[0].server_number)
            .unwrap();

        println!("{:#?}", config);
        assert!(config.is_some());
    }

    #[test]
    #[serial(boot_configuration)]
    fn set_server_rescue_boot_configuration() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);

        let config = robot
            .get_server_rescue_boot_configuration(servers[0].server_number)
            .unwrap()
            .unwrap();

        robot
            .set_server_rescue_boot_configuration(
                servers[0].server_number,
                &config.os[0],
                Some(config.arch[0]),
                &[],
            )
            .unwrap();

        let config = robot
            .get_server_rescue_boot_configuration(servers[0].server_number)
            .unwrap()
            .unwrap();

        assert!(config.active);
        assert_eq!(config.os, vec!["linux"]);
        assert_eq!(config.arch, vec![64]);

        robot
            .delete_server_rescue_boot_configuration(servers[0].server_number)
            .unwrap();
    }

    #[test]
    #[serial(boot_configuration)]
    fn set_server_linux_boot_configuration() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);

        let config = robot
            .get_server_linux_boot_configuration(servers[0].server_number)
            .unwrap()
            .unwrap();

        robot
            .set_server_linux_boot_configuration(
                servers[0].server_number,
                &config.dist[0],
                Some(config.arch[0]),
                &config.lang[0],
                &[],
            )
            .unwrap();

        let config = robot
            .get_server_linux_boot_configuration(servers[0].server_number)
            .unwrap()
            .unwrap();

        assert!(config.active);
        assert_eq!(config.dist, vec![config.dist[0].clone()]);
        assert_eq!(config.arch, vec![config.arch[0]]);
        assert_eq!(config.lang, vec![config.lang[0].clone()]);

        robot
            .delete_server_linux_boot_configuration(servers[0].server_number)
            .unwrap();
    }

    #[test]
    #[serial(boot_configuration)]
    fn set_server_vnc_boot_configuration() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);

        let config = robot
            .get_server_vnc_boot_configuration(servers[0].server_number)
            .unwrap()
            .unwrap();

        robot
            .set_server_vnc_boot_configuration(
                servers[0].server_number,
                &config.dist[0],
                Some(config.arch[0]),
                &config.lang[0],
            )
            .unwrap();

        let config = robot
            .get_server_vnc_boot_configuration(servers[0].server_number)
            .unwrap()
            .unwrap();

        assert!(config.active);
        assert_eq!(config.dist, vec![config.dist[0].clone()]);
        assert_eq!(config.arch, vec![config.arch[0]]);
        assert_eq!(config.lang, vec![config.lang[0].clone()]);

        robot
            .delete_server_vnc_boot_configuration(servers[0].server_number)
            .unwrap();
    }
}
