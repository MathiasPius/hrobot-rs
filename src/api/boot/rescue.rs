use std::borrow::Cow;
use std::fmt::Display;

use crate::api::server::ServerId;
use crate::{error::Error, AsyncRobot};

use crate::api::{wrapper::Single, UnauthenticatedRequest};

fn get_rescue_config(server_number: ServerId) -> UnauthenticatedRequest<Single<Rescue>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/rescue"
    ))
}

fn enable_rescue_config(
    server_number: ServerId,
    rescue: RescueConfig,
) -> Result<UnauthenticatedRequest<Single<ActiveRescueConfig>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/rescue"
    ))
    .with_method("POST")
    .with_body(rescue)
}

fn disable_rescue_config(
    server_number: ServerId,
) -> UnauthenticatedRequest<Single<AvailableRescueConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/rescue"
    ))
    .with_method("DELETE")
}

fn get_last_rescue_config(
    server_number: ServerId,
) -> UnauthenticatedRequest<Single<ActiveRescueConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/rescue/last"
    ))
}

impl AsyncRobot {
    /// Retrieve a [`Server`](crate::api::server::Server)'s [`ActiveRescueConfig`] configuration,
    /// or a list of available operating systems, if the rescue
    /// system is not currently active.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::boot::{Rescue, ActiveRescueConfig, AvailableRescueConfig};
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// match robot.get_rescue_config(ServerId(1234567)).await.unwrap() {
    ///     Rescue::Active(ActiveRescueConfig { operating_system, .. }) => {
    ///         println!("currently active rescue system is: {operating_system}");
    ///         // e.g.: currently active rescue system is: vkvm
    ///     },
    ///     Rescue::Available(AvailableRescueConfig { operating_systems, .. }) => {
    ///         println!("available rescue systems are: {:?}", operating_systems)
    ///         // e.g.: available rescue systems are: linux, linuxold, vkvm
    ///     }
    /// }
    /// # }
    /// ```
    pub async fn get_rescue_config(&self, server_number: ServerId) -> Result<Rescue, Error> {
        Ok(self.go(get_rescue_config(server_number)).await?.0)
    }

    /// Get the last [`ActiveRescueConfig`].
    ///
    /// This is the last configuration that was active on the server,
    /// not the *currently* active configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_last_rescue_config(ServerId(1234567)).await.unwrap();
    /// # }
    /// ```
    pub async fn get_last_rescue_config(
        &self,
        server_number: ServerId,
    ) -> Result<ActiveRescueConfig, Error> {
        Ok(self.go(get_last_rescue_config(server_number)).await?.0)
    }

    /// Enable a rescue configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # use hrobot::api::boot::{Rescue, RescueConfig, Keyboard, RescueOperatingSystem};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_rescue_config(ServerId(1234567), RescueConfig {
    ///     operating_system: RescueOperatingSystem::from("vkvm"),
    ///     authorized_keys: vec!["d7:34:1c:8c:4e:20:e0:1f:07:66:45:d9:97:22:ec:07".to_string()],
    ///     keyboard: Keyboard::German,
    /// }).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_rescue_config(
        &self,
        server_number: ServerId,
        config: RescueConfig,
    ) -> Result<ActiveRescueConfig, Error> {
        Ok(self
            .go(enable_rescue_config(server_number, config)?)
            .await?
            .0)
    }

    /// Disable the active rescue configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_rescue_config(ServerId(1234567)).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_rescue_config(
        &self,
        server_number: ServerId,
    ) -> Result<AvailableRescueConfig, Error> {
        Ok(self.go(disable_rescue_config(server_number)).await?.0)
    }
}

use serde::{Deserialize, Serialize};

use crate::api::keys::SshKeyReference;

/// Keyboard layout.
///
/// Defaults to US.
#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keyboard {
    /// US layout
    #[default]
    #[serde(rename = "us")]
    US,
    /// UK layout
    #[serde(rename = "uk")]
    UK,
    /// Swiss layout
    #[serde(rename = "ch")]
    Swiss,
    /// German layout
    #[serde(rename = "de")]
    German,
    /// Finnish layout
    #[serde(rename = "fi")]
    Finnish,
    /// French layout
    #[serde(rename = "fr")]
    French,
    /// Japanese layout.
    #[serde(rename = "jp")]
    Japanese,
    /// Other layout.
    #[serde(untagged)]
    Other(String),
}

/// Configuration of the rescue system to enable.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RescueConfig {
    /// Rescue operating system to activate.
    #[serde(rename = "os")]
    pub operating_system: RescueOperatingSystem,

    /// Key fingerprints to authorize for server access.
    #[serde(rename = "authorized_key", skip_serializing_if = "Vec::is_empty")]
    pub authorized_keys: Vec<String>,

    /// Keyboard layout to use for the rescue system.
    ///
    /// Defaults to US.
    pub keyboard: Keyboard,
}

/// Currently active rescue system configuration.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ActiveRescueConfig {
    /// Active rescue operating system.
    #[serde(rename = "os")]
    pub operating_system: RescueOperatingSystem,

    /// Root password for the currently active rescue system.
    pub password: Option<String>,

    /// Rescue system host keys
    #[serde(rename = "host_key")]
    pub host_keys: Vec<String>,

    /// Keys authorized to access the rescue system via SSH.
    #[serde(
        rename = "authorized_key",
        deserialize_with = "crate::api::wrapper::deserialize_inner_vec"
    )]
    pub authorized_keys: Vec<SshKeyReference>,
}

/// Available rescue system configurations
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AvailableRescueConfig {
    /// Available rescue operating systems.
    #[serde(rename = "os")]
    pub operating_systems: Vec<RescueOperatingSystem>,
}

/// Represents the currently active rescue configuration,
/// or if inactive, the available rescue systems.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Rescue {
    /// Currently active rescue system
    Active(ActiveRescueConfig),

    /// Available rescue system configurations
    Available(AvailableRescueConfig),
}

/// Rescue Distribution, e.g. "vkvm".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RescueOperatingSystem(pub Cow<'static, str>);

impl From<String> for RescueOperatingSystem {
    fn from(value: String) -> Self {
        RescueOperatingSystem(Cow::from(value))
    }
}

impl From<&'static str> for RescueOperatingSystem {
    fn from(value: &'static str) -> Self {
        RescueOperatingSystem(Cow::from(value))
    }
}

impl Display for RescueOperatingSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<str> for RescueOperatingSystem {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

#[cfg(test)]
mod isolated_tests {
    use crate::api::boot::Keyboard;

    #[test]
    fn serialize_keyboard() {
        let german = Keyboard::German;
        let danish = Keyboard::Other("da".to_string());

        assert_eq!(serde_json::to_string(&german).unwrap(), r#""de""#);
        assert_eq!(serde_json::to_string(&danish).unwrap(), r#""da""#);
    }
}
