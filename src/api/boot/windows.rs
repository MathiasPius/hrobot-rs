use std::borrow::Cow;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::api::server::ServerId;
use crate::api::{wrapper::Single, UnauthenticatedRequest};
use crate::{error::Error, AsyncRobot};

fn get_windows_config(server_number: ServerId) -> UnauthenticatedRequest<Single<Windows>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/windows"
    ))
}

fn enable_windows_config(
    server_number: ServerId,
    config: WindowsConfig,
) -> Result<UnauthenticatedRequest<Single<ActiveWindowsConfig>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/windows"
    ))
    .with_method("POST")
    .with_body(config)
}

fn disable_windows_config(
    server_number: ServerId,
) -> UnauthenticatedRequest<Single<AvailableWindowsConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/windows"
    ))
    .with_method("DELETE")
}

fn get_last_windows_config(
    server_number: ServerId,
) -> UnauthenticatedRequest<Single<ActiveWindowsConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/windows/last"
    ))
}

impl AsyncRobot {
    /// Retrieve a [`Server`](crate::api::server::Server)'s [`ActiveWindowsConfig`] configuration,
    /// or a list of available distributions and languages, if the Windows installation system
    /// is not currently active.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::boot::{Windows, ActiveWindowsConfig, AvailableWindowsConfig};
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// match robot.get_windows_config(ServerId(1234567)).await.unwrap() {
    ///     Windows::Active(ActiveWindowsConfig { distribution, .. }) => {
    ///         println!("currently active windows installation distribution is: {distribution}");
    ///         // e.g.: currently active rescue system is: vkvm
    ///     },
    ///     Windows::Available(AvailableWindowsConfig { distributions, .. }) => {
    ///         println!("available windows installation distributions are: {:?}", distributions)
    ///         // e.g.: available rescue systems are: linux, linuxold, vkvm
    ///     }
    /// }
    /// # }
    /// ```
    pub async fn get_windows_config(&self, server_number: ServerId) -> Result<Windows, Error> {
        Ok(self.go(get_windows_config(server_number)).await?.0)
    }

    /// Get the last [`ActiveWindowsConfig`].
    ///
    /// This is the last configuration that was active on the server,
    /// not the *currently* active configuration.
    ///
    /// **Warning**: This is an undocumented part of the Hetzner Robot API
    /// and *may* stop working at any time, without warning.
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
    pub async fn get_last_windows_config(
        &self,
        server_number: ServerId,
    ) -> Result<ActiveWindowsConfig, Error> {
        Ok(self.go(get_last_windows_config(server_number)).await?.0)
    }

    /// Enable the Windows installation system.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # use hrobot::api::boot::{WindowsConfig, WindowsDistribution};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_windows_config(ServerId(1234567), WindowsConfig {
    ///     distribution: WindowsDistribution::from("standard"),
    ///     language: "en".to_string()
    /// }).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_windows_config(
        &self,
        server_number: ServerId,
        config: WindowsConfig,
    ) -> Result<ActiveWindowsConfig, Error> {
        Ok(self
            .go(enable_windows_config(server_number, config)?)
            .await?
            .0)
    }

    /// Disable the active Windows installation configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_windows_config(ServerId(1234567)).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_windows_config(
        &self,
        server_number: ServerId,
    ) -> Result<AvailableWindowsConfig, Error> {
        Ok(self.go(disable_windows_config(server_number)).await?.0)
    }
}

/// Currently active Windows installation configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActiveWindowsConfig {
    /// Active Windows installation distribution.
    #[serde(rename = "dist")]
    pub distribution: WindowsDistribution,

    /// Active Windows installation language.
    #[serde(rename = "lang")]
    pub language: String,

    /// Administrator password for the currently active
    /// Windows installation configuration.
    pub password: Option<String>,
}

/// availble Windows installation configuration options.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AvailableWindowsConfig {
    /// Available Windows installation distributions.
    #[serde(rename = "dist")]
    pub distributions: Vec<WindowsDistribution>,

    /// Available Windows installation languages.
    #[serde(rename = "lang")]
    pub language: Vec<String>,
}

/// Describes either the active or available Windows configurations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Windows {
    /// Currently active Windows installation configuration.
    Active(ActiveWindowsConfig),
    /// Describes availble Windows installation configuration options.
    Available(AvailableWindowsConfig),
}

/// Aplicable Windows boot configuration.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WindowsConfig {
    /// Distribution of Windows to install.
    #[serde(rename = "dist")]
    pub distribution: WindowsDistribution,

    /// Language to install.
    #[serde(rename = "lang")]
    pub language: String,
}

/// Windows Distribution, e.g. "standard".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowsDistribution(pub Cow<'static, str>);

impl From<String> for WindowsDistribution {
    fn from(value: String) -> Self {
        WindowsDistribution(Cow::from(value))
    }
}

impl From<&'static str> for WindowsDistribution {
    fn from(value: &'static str) -> Self {
        WindowsDistribution(Cow::from(value))
    }
}

impl Display for WindowsDistribution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<str> for WindowsDistribution {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}
