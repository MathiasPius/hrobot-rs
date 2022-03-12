use std::net::{Ipv4Addr, Ipv6Addr};

use serde::{Deserialize, Serialize};

use crate::{Error, SyncRobot};

#[derive(Debug, Deserialize)]
#[serde(from = "AuthorizedKeyResponse")]
pub struct AuthorizedKey {
    pub name: String,
    pub fingerprint: String,
    pub key_type: String,
    pub size: u32,
        }

#[derive(Debug, Deserialize)]
struct AuthorizedKeyInner {
    pub name: String,
    pub fingerprint: String,
    #[serde(rename = "type")]
    pub key_type: String,
    pub size: u32,
}

#[derive(Debug, Deserialize)]
struct AuthorizedKeyResponse {
    pub key: AuthorizedKeyInner,
}

impl From<AuthorizedKeyResponse> for AuthorizedKey {
    fn from(key: AuthorizedKeyResponse) -> AuthorizedKey {
        AuthorizedKey {
            name: key.key.name,
            fingerprint: key.key.fingerprint,
            key_type: key.key.key_type,
            size: key.key.size,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RescueConfiguration {
    #[serde(rename = "server_ip")]
    pub ipv4: Option<Ipv4Addr>,
    #[serde(rename = "server_ipv6_net")]
    pub ipv6_net: Ipv6Addr,
    #[serde(rename = "server_number")]
    pub id: u32,
    #[serde(deserialize_with = "crate::string_or_seq_string")]
    pub os: Vec<String>,
    #[serde(deserialize_with = "crate::num_or_seq_num")]
    pub arch: Vec<u64>,
    pub active: bool,
    pub password: Option<String>,
    pub authorized_key: Vec<AuthorizedKey>,
    pub host_key: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct LinuxConfiguration {
    #[serde(rename = "server_ip")]
    pub ipv4: Option<Ipv4Addr>,
    #[serde(rename = "server_ipv6_net")]
    pub ipv6_net: Ipv6Addr,
    #[serde(rename = "server_number")]
    pub id: u32,
    #[serde(deserialize_with = "crate::string_or_seq_string")]
    pub dist: Vec<String>,
    #[serde(deserialize_with = "crate::num_or_seq_num")]
    pub arch: Vec<u64>,
    #[serde(deserialize_with = "crate::string_or_seq_string")]
    pub lang: Vec<String>,
    pub active: bool,
    pub password: Option<String>,
    pub authorized_key: Vec<AuthorizedKey>,
    pub host_key: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct VncConfiguration {
    #[serde(rename = "server_ip")]
    pub ipv4: Option<Ipv4Addr>,
    #[serde(rename = "server_ipv6_net")]
    pub ipv6_net: Ipv6Addr,
    #[serde(rename = "server_number")]
    pub id: u32,
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
    #[serde(rename = "server_ip")]
    pub ipv4: Option<Ipv4Addr>,
    #[serde(rename = "server_ipv6_net")]
    pub ipv6_net: Ipv6Addr,
    #[serde(rename = "server_number")]
    pub id: u32,
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
    #[serde(rename = "server_ip")]
    pub ipv4: Option<Ipv4Addr>,
    #[serde(rename = "server_ipv6_net")]
    pub ipv6_net: Ipv6Addr,
    #[serde(rename = "server_number")]
    pub id: u32,
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
    #[serde(rename = "server_ip")]
    pub ipv4: Option<Ipv4Addr>,
    #[serde(rename = "server_ipv6_net")]
    pub ipv6_net: Ipv6Addr,
    #[serde(rename = "server_number")]
    pub id: u32,
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

impl From<BootConfigurationResponse> for BootConfiguration {
    fn from(b: BootConfigurationResponse) -> Self {
        b.boot
    }
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

impl<T> BootRobot for T
where
    T: SyncRobot,
{
    fn list_server_boot_configurations(
        &self,
        server_number: u32,
    ) -> Result<BootConfiguration, Error> {
        self.get::<BootConfigurationResponse>(&format!("/boot/{}", server_number))
            .map(BootConfiguration::from)
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
        Ok(self
            .get::<BootConfiguration>(&format!("/boot/{}/rescue", server_number))?
            .rescue)
    }

    fn delete_server_rescue_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<RescueConfiguration>, Error> {
        Ok(self
            .delete::<BootConfiguration, ()>(&format!("/boot/{}/rescue", server_number), ())?
            .rescue)
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

        Ok(self
            .post::<BootConfiguration, SetRescueConfigurationRequest>(
                &format!("/boot/{}/rescue", server_number),
                SetRescueConfigurationRequest {
                    os,
                    arch,
                    authorized_key: authorized_keys,
                },
            )?
            .rescue)
    }

    fn get_server_linux_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<LinuxConfiguration>, Error> {
        Ok(self
            .get::<BootConfiguration>(&format!("/boot/{}/linux", server_number))?
            .linux)
    }

    fn delete_server_linux_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<LinuxConfiguration>, Error> {
        Ok(self
            .delete::<BootConfiguration, ()>(&format!("/boot/{}/linux", server_number), ())?
            .linux)
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

        Ok(self
            .post::<BootConfiguration, SetLinuxConfigurationRequest>(
                &format!("/boot/{}/linux", server_number),
                SetLinuxConfigurationRequest {
                    dist,
                    lang,
                    arch,
                    authorized_key: authorized_keys,
                },
            )?
            .linux)
    }

    fn get_server_vnc_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<VncConfiguration>, Error> {
        Ok(self
            .get::<BootConfiguration>(&format!("/boot/{}/vnc", server_number))?
            .vnc)
    }

    fn delete_server_vnc_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<VncConfiguration>, Error> {
        Ok(self
            .delete::<BootConfiguration, ()>(&format!("/boot/{}/vnc", server_number), ())?
            .vnc)
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

        Ok(self
            .post::<BootConfiguration, SetVncConfigurationRequest>(
                &format!("/boot/{}/vnc", server_number),
                SetVncConfigurationRequest { dist, lang, arch },
            )?
            .vnc)
    }

    fn get_server_windows_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<WindowsConfiguration>, Error> {
        Ok(self
            .get::<BootConfiguration>(&format!("/boot/{}/windows", server_number))?
            .windows)
    }

    fn delete_server_windows_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<WindowsConfiguration>, Error> {
        Ok(self
            .delete::<BootConfiguration, ()>(&format!("/boot/{}/windows", server_number), ())?
            .windows)
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

        Ok(self
            .post::<BootConfiguration, SetWindowsConfigurationRequest>(
                &format!("/boot/{}/windows", server_number),
                SetWindowsConfigurationRequest { lang },
            )?
            .windows)
    }

    fn get_server_plesk_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<PleskConfiguration>, Error> {
        Ok(self
            .get::<BootConfiguration>(&format!("/boot/{}/plesk", server_number))?
            .plesk)
    }

    fn delete_server_plesk_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<PleskConfiguration>, Error> {
        Ok(self
            .delete::<BootConfiguration, ()>(&format!("/boot/{}/plesk", server_number), ())?
            .plesk)
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

        Ok(self
            .post::<BootConfiguration, SetPleskConfigurationRequest>(
                &format!("/boot/{}/plesk", server_number),
                SetPleskConfigurationRequest {
                    dist,
                    lang,
                    arch,
                    hostname,
                },
            )?
            .plesk)
    }

    fn get_server_cpanel_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<CPanelConfiguration>, Error> {
        Ok(self
            .get::<BootConfiguration>(&format!("/boot/{}/cpanel", server_number))?
            .cpanel)
    }

    fn delete_server_cpanel_boot_configuration(
        &self,
        server_number: u32,
    ) -> Result<Option<CPanelConfiguration>, Error> {
        Ok(self
            .delete::<BootConfiguration, ()>(&format!("/boot/{}/cpanel", server_number), ())?
            .cpanel)
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

        Ok(self
            .post::<BootConfiguration, SetCpanelConfigurationRequest>(
                &format!("/boot/{}/cpanel", server_number),
                SetCpanelConfigurationRequest {
                    dist,
                    lang,
                    arch,
                    hostname,
                },
            )?
            .cpanel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Robot, ServerRobot};
    use serial_test::serial;

    #[test]
    #[ignore]
    fn list_server_boot_configuration() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);

        let configs = robot.list_server_boot_configurations(servers[0].id);

        assert!(configs.is_ok());
        println!("{:#?}", configs.unwrap());
    }

    #[test]
    #[ignore]
    fn get_server_rescue_boot_configuration() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);

        let config = robot
            .get_server_rescue_boot_configuration(servers[0].id)
            .unwrap();

        println!("{:#?}", config);
        assert!(config.is_some());
    }

    #[test]
    #[ignore]
    #[serial(boot_configuration)]
    fn set_server_rescue_boot_configuration() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);

        let config = robot
            .get_server_rescue_boot_configuration(servers[0].id)
            .unwrap()
            .unwrap();

        robot
            .set_server_rescue_boot_configuration(
                servers[0].id,
                &config.os[0],
                Some(config.arch[0]),
                &[],
            )
            .unwrap();

        let config = robot
            .get_server_rescue_boot_configuration(servers[0].id)
            .unwrap()
            .unwrap();

        assert!(config.active);
        assert_eq!(config.os, vec!["linux"]);
        assert_eq!(config.arch, vec![64]);

        robot
            .delete_server_rescue_boot_configuration(servers[0].id)
            .unwrap();
    }

    #[test]
    #[ignore]
    #[serial(boot_configuration)]
    fn set_server_linux_boot_configuration() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);

        let config = robot
            .get_server_linux_boot_configuration(servers[0].id)
            .unwrap()
            .unwrap();

        robot
            .set_server_linux_boot_configuration(
                servers[0].id,
                &config.dist[0],
                Some(config.arch[0]),
                &config.lang[0],
                &[],
            )
            .unwrap();

        let config = robot
            .get_server_linux_boot_configuration(servers[0].id)
            .unwrap()
            .unwrap();

        assert!(config.active);
        assert_eq!(config.dist, vec![config.dist[0].clone()]);
        assert_eq!(config.arch, vec![config.arch[0]]);
        assert_eq!(config.lang, vec![config.lang[0].clone()]);

        robot
            .delete_server_linux_boot_configuration(servers[0].id)
            .unwrap();
    }

    #[test]
    #[ignore]
    #[serial(boot_configuration)]
    fn set_server_vnc_boot_configuration() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);

        let config = robot
            .get_server_vnc_boot_configuration(servers[0].id)
            .unwrap()
            .unwrap();

        robot
            .set_server_vnc_boot_configuration(
                servers[0].id,
                &config.dist[0],
                Some(config.arch[0]),
                &config.lang[0],
            )
            .unwrap();

        let config = robot
            .get_server_vnc_boot_configuration(servers[0].id)
            .unwrap()
            .unwrap();

        assert!(config.active);
        assert_eq!(config.dist, vec![config.dist[0].clone()]);
        assert_eq!(config.arch, vec![config.arch[0]]);
        assert_eq!(config.lang, vec![config.lang[0].clone()]);

        robot
            .delete_server_vnc_boot_configuration(servers[0].id)
            .unwrap();
    }
}
