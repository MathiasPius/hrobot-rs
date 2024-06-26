use std::{borrow::Cow, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{
    api::{server::ServerId, wrapper::Single, UnauthenticatedRequest},
    error::Error,
    AsyncRobot,
};

fn get_plesk_config(server_number: ServerId) -> UnauthenticatedRequest<Single<Plesk>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/plesk"
    ))
}

fn enable_plesk_config(
    server_number: ServerId,
    config: PleskConfig,
) -> Result<UnauthenticatedRequest<Single<ActivePleskConfig>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/plesk"
    ))
    .with_method("POST")
    .with_body(config)
}

fn disable_plesk_config(
    server_number: ServerId,
) -> UnauthenticatedRequest<Single<AvailablePleskConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/plesk"
    ))
    .with_method("DELETE")
}

fn get_last_plesk_config(
    server_number: ServerId,
) -> UnauthenticatedRequest<Single<ActivePleskConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/plesk/last"
    ))
}

impl AsyncRobot {
    /// Retrieve a [`Server`](crate::api::server::Server)'s [`ActivePleskConfig`]
    /// configuration, or a list of available distributions and languages,
    /// if the Plesk installation system is not currently active.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::boot::{Plesk, ActivePleskConfig, AvailablePleskConfig};
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// match robot.get_plesk_config(ServerId(1234567)).await.unwrap() {
    ///     Plesk::Active(ActivePleskConfig { distribution, .. }) => {
    ///         println!("currently active plesk distribution is: {distribution}");
    ///         // e.g.: currently active plesk distribution is: CentOS-Stream
    ///     },
    ///     Plesk::Available(AvailablePleskConfig { distributions, .. }) => {
    ///         println!("available plesk distributions are: {:?}", distributions)
    ///         // e.g.: available plesk distributions are: CentOS-Stream, ...
    ///     }
    /// }
    /// # }
    /// ```
    pub async fn get_plesk_config(&self, server_number: ServerId) -> Result<Plesk, Error> {
        Ok(self.go(get_plesk_config(server_number)).await?.0)
    }

    /// Get the last [`ActivePleskConfig`].
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
    /// robot.get_last_plesk_config(ServerId(1234567)).await.unwrap();
    /// # }
    /// ```
    pub async fn get_last_plesk_config(
        &self,
        server_number: ServerId,
    ) -> Result<ActivePleskConfig, Error> {
        Ok(self.go(get_last_plesk_config(server_number)).await?.0)
    }

    /// Enable a linux installation configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # use hrobot::api::boot::{Plesk, PleskConfig, PleskDistribution};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_plesk_config(ServerId(1234567), PleskConfig {
    ///     distribution: PleskDistribution::from("CentOS-Stream"),
    ///     language: "en_US".to_string(),
    ///     hostname: "plesk.example.com".to_string(),
    /// }).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_plesk_config(
        &self,
        server_number: ServerId,
        config: PleskConfig,
    ) -> Result<ActivePleskConfig, Error> {
        Ok(self
            .go(enable_plesk_config(server_number, config)?)
            .await?
            .0)
    }

    /// Disable the active Plesk installation configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_plesk_config(ServerId(1234567)).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_plesk_config(
        &self,
        server_number: ServerId,
    ) -> Result<AvailablePleskConfig, Error> {
        Ok(self.go(disable_plesk_config(server_number)).await?.0)
    }
}

/// Applicable Plesk boot configuration.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PleskConfig {
    /// Distribution for the Plesk installation.
    #[serde(rename = "dist")]
    pub distribution: PleskDistribution,

    /// Hostname for the Plesk installation.
    pub hostname: String,

    /// Language of the distribution
    #[serde(rename = "lang")]
    pub language: String,
}

/// Describes available Plesk configuration options.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AvailablePleskConfig {
    /// Available distributions for Plesk installation.
    #[serde(rename = "dist")]
    pub distributions: Vec<PleskDistribution>,

    /// Available languages for the Plesk installation.
    #[serde(rename = "lang")]
    pub languages: Vec<String>,
}

/// Currently active Plesk configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActivePleskConfig {
    /// Distribution selected in currently active Plesk installation.
    #[serde(rename = "dist")]
    pub distribution: PleskDistribution,

    /// Language selected in currently active Plesk installation.
    #[serde(rename = "lang")]
    pub language: String,

    /// Password for the Plesk installation.
    pub password: Option<String>,

    /// Hostname for the Plesk installation.
    pub hostname: Option<String>,
}

/// Describes either the active or available Plesk installation configurations.
///
/// If a Plesk installation system is active, it ([`ActivePleskConfig`]) will be returned,
/// otherwise a struct ([`AvailablePleskConfig`]) representing the available Plesk distributions
/// and languages is returned.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Plesk {
    /// Currently active Plesk configuration.
    Active(ActivePleskConfig),
    /// Describes available Plesk configuration options.
    Available(AvailablePleskConfig),
}

/// Plesk Distribution, e.g. "CentOS-Stream".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PleskDistribution(pub Cow<'static, str>);

impl From<String> for PleskDistribution {
    fn from(value: String) -> Self {
        PleskDistribution(Cow::from(value))
    }
}

impl From<&'static str> for PleskDistribution {
    fn from(value: &'static str) -> Self {
        PleskDistribution(Cow::from(value))
    }
}

impl Display for PleskDistribution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<str> for PleskDistribution {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}
