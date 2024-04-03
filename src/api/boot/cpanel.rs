use std::{borrow::Cow, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{
    api::{server::ServerId, wrapper::Single, UnauthenticatedRequest},
    error::Error,
    AsyncRobot,
};

fn get_cpanel_config(server_number: ServerId) -> UnauthenticatedRequest<Single<Cpanel>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/cpanel"
    ))
}

fn enable_cpanel_config(
    server_number: ServerId,
    config: CpanelConfig,
) -> Result<UnauthenticatedRequest<Single<ActiveCpanelConfig>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/cpanel"
    ))
    .with_method("POST")
    .with_body(config)
}

fn disable_cpanel_config(
    server_number: ServerId,
) -> UnauthenticatedRequest<Single<AvailableCpanelConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/cpanel"
    ))
    .with_method("DELETE")
}

fn get_last_cpanel_config(
    server_number: ServerId,
) -> UnauthenticatedRequest<Single<ActiveCpanelConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/cpanel/last"
    ))
}

impl AsyncRobot {
    /// Retrieve a [`Server`](crate::api::server::Server)'s [`ActiveCpanelConfig`]
    /// configuration, or a list of available distributions and languages,
    /// if the Cpanel installation system is not currently active.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # use hrobot::api::boot::{
    /// #   Cpanel, ActiveCpanelConfig, AvailableCpanelConfig,
    /// # };
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// match robot.get_cpanel_config(ServerId(1234567)).await.unwrap() {
    ///     Cpanel::Active(ActiveCpanelConfig { distribution, .. }) => {
    ///         println!("currently active cpanel distribution is: {distribution}");
    ///         // e.g.: currently active cpanel distribution is: CentOS-Stream
    ///     },
    ///     Cpanel::Available(AvailableCpanelConfig { distributions, .. }) => {
    ///         println!("available cpanel distributions are: {:?}", distributions)
    ///         // e.g.: available cpanel distributions are: CentOS-Stream, ...
    ///     }
    /// }
    /// # }
    /// ```
    pub async fn get_cpanel_config(&self, server_number: ServerId) -> Result<Cpanel, Error> {
        Ok(self.go(get_cpanel_config(server_number)).await?.0)
    }

    /// Get the last [`ActiveCpanelConfig`].
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
    /// robot.get_last_cpanel_config(ServerId(1234567)).await.unwrap();
    /// # }
    /// ```
    pub async fn get_last_cpanel_config(
        &self,
        server_number: ServerId,
    ) -> Result<ActiveCpanelConfig, Error> {
        Ok(self.go(get_last_cpanel_config(server_number)).await?.0)
    }

    /// Enable a linux installation configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::boot::{Cpanel, CpanelConfig, CpanelDistribution};
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_cpanel_config(ServerId(1234567), CpanelConfig {
    ///     distribution: CpanelDistribution::from("CentOS-Stream"),
    ///     language: "en_US".to_string(),
    ///     hostname: "cpanel.example.com".to_string(),
    /// }).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_cpanel_config(
        &self,
        server_number: ServerId,
        config: CpanelConfig,
    ) -> Result<ActiveCpanelConfig, Error> {
        Ok(self
            .go(enable_cpanel_config(server_number, config)?)
            .await?
            .0)
    }

    /// Disable the active Cpanel installation configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_cpanel_config(ServerId(1234567)).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_cpanel_config(
        &self,
        server_number: ServerId,
    ) -> Result<AvailableCpanelConfig, Error> {
        Ok(self.go(disable_cpanel_config(server_number)).await?.0)
    }
}

/// Applicable CPanel configuration.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CpanelConfig {
    /// Distribution for the Cpanel installation.
    #[serde(rename = "dist")]
    pub distribution: CpanelDistribution,

    /// Hostname for the Cpanel installation.
    pub hostname: String,

    /// Language of the distribution
    #[serde(rename = "lang")]
    pub language: String,
}

/// Describes available Cpanel configuration options.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AvailableCpanelConfig {
    /// Available distributions for Cpanel installation.
    #[serde(rename = "dist")]
    pub distributions: Vec<CpanelDistribution>,

    /// Available languages for the Cpanel installation.
    #[serde(rename = "lang")]
    pub languages: Vec<String>,
}

/// Currently active Cpanel configuration.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ActiveCpanelConfig {
    /// Distribution selected in currently active Cpanel installation.
    #[serde(rename = "dist")]
    pub distribution: CpanelDistribution,

    /// Language selected in currently active Cpanel installation.
    #[serde(rename = "lang")]
    pub language: String,

    /// Password for the Cpanel installation.
    pub password: Option<String>,

    /// Hostname for the Cpanel installation.
    pub hostname: Option<String>,
}

/// Describes either the active or available Cpanel installation configurations.
///
/// If a Cpanel installation system is active, it ([`ActiveCpanelConfig`]) will be returned,
/// otherwise a struct ([`AvailableCpanelConfig`]) representing the available Cpanel distributions
/// and languages is returned.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Cpanel {
    /// Currently active Cpanel configuration.
    Active(ActiveCpanelConfig),
    /// Describes available Cpanel configuration options.
    Available(AvailableCpanelConfig),
}

/// CPanel Distribution, e.g. "CentOS-Stream".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CpanelDistribution(pub Cow<'static, str>);

impl From<String> for CpanelDistribution {
    fn from(value: String) -> Self {
        CpanelDistribution(Cow::from(value))
    }
}

impl From<&'static str> for CpanelDistribution {
    fn from(value: &'static str) -> Self {
        CpanelDistribution(Cow::from(value))
    }
}

impl Display for CpanelDistribution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<str> for CpanelDistribution {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}
