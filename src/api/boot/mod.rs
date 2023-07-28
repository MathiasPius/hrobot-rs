//! Boot Configuration structs and implementation.

mod cpanel;
mod linux;
mod plesk;
mod rescue;
mod vnc;
mod windows;

pub use cpanel::*;
pub use linux::*;
pub use plesk::*;
pub use rescue::*;
pub use vnc::*;
pub use windows::*;

use crate::{
    api::{wrapper::Single, UnauthenticatedRequest},
    error::Error,
    AsyncRobot,
};
use serde::Deserialize;

use super::server::ServerId;

/// Describes the status of each of the available boot configuration systems.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Active or available rescue system configurations.
    pub rescue: Option<Rescue>,

    /// Active or available Linux installation configurations.
    pub linux: Option<Linux>,

    /// Active or available VNC installation configurations.
    pub vnc: Option<Vnc>,

    /// Active or available Windows installation configurations.
    pub windows: Option<Windows>,

    /// Active or available Plesk installation configurations.
    pub plesk: Option<Plesk>,

    /// Active or available CPanel installation configurations.
    pub cpanel: Option<Cpanel>,
}

/// Contains only the currently active boot configuration system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActiveConfig {
    /// Active rescue system configuration.
    Rescue(ActiveRescueConfig),
    /// Active Linux installation configuration.
    Linux(ActiveLinuxConfig),
    /// Active VNC installation configuration.
    Vnc(ActiveVncConfig),
    /// Active Windows installation configuration.
    Windows(ActiveWindowsConfig),
    /// Active Plesk installation configuration.
    Plesk(ActivePleskConfig),
    /// Active CPanel installation configuration.
    CPanel(ActiveCpanelConfig),
}

impl Config {
    /// Retrieve the currently active configuration, if any.
    pub fn active(&self) -> Option<ActiveConfig> {
        if let Some(Rescue::Active(config)) = &self.rescue {
            return Some(ActiveConfig::Rescue(config.clone()));
        }

        if let Some(Linux::Active(config)) = &self.linux {
            return Some(ActiveConfig::Linux(config.clone()));
        }

        if let Some(Vnc::Active(config)) = &self.vnc {
            return Some(ActiveConfig::Vnc(config.clone()));
        }

        if let Some(Windows::Active(config)) = &self.windows {
            return Some(ActiveConfig::Windows(config.clone()));
        }

        if let Some(Plesk::Active(config)) = &self.plesk {
            return Some(ActiveConfig::Plesk(config.clone()));
        }

        if let Some(Cpanel::Active(config)) = &self.cpanel {
            return Some(ActiveConfig::CPanel(config.clone()));
        }

        None
    }
}

fn get_config(server_number: ServerId) -> UnauthenticatedRequest<Single<Config>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}"
    ))
}

impl AsyncRobot {
    /// Retrieve the status of all boot configuration systems,
    /// whether active or available or a server.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// let config = robot.get_boot_config(ServerId(1234567)).await.unwrap();
    /// // Make sure no boot configurations are currently active.
    /// assert!(config.active().is_none());
    /// # }
    /// ```
    pub async fn get_boot_config(&self, server_number: ServerId) -> Result<Config, Error> {
        Ok(self.go(get_config(server_number)).await?.0)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "non-disruptive-tests")]
    mod non_disruptive_tests {
        use serial_test::serial;
        use tracing::info;
        use tracing_test::traced_test;

        #[tokio::test]
        #[traced_test]
        #[serial("boot-configuration")]
        async fn test_get_boot_configuration() {
            let _ = dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let servers = robot.list_servers().await.unwrap();
            info!("{servers:#?}");

            if let Some(server) = servers.first() {
                let config = robot.get_boot_config(server.id).await.unwrap();
                info!("{config:#?}");
            }
        }
    }
}
