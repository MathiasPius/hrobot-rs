use serde::{Deserialize, Serialize};

use crate::{
    api::{wrapper::Single, UnauthenticatedRequest},
    error::Error,
    AsyncHttpClient, AsyncRobot,
};

fn get_cpanel_config(server_number: u32) -> UnauthenticatedRequest<Single<Cpanel>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/cpanel"
    ))
}

fn enable_cpanel_config(
    server_number: u32,
    config: CpanelConfig,
) -> Result<UnauthenticatedRequest<Single<ActiveCpanelConfig>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/cpanel"
    ))
    .with_method("POST")
    .with_body(config)
}

fn disable_cpanel_config(
    server_number: u32,
) -> UnauthenticatedRequest<Single<AvailableCpanelConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/cpanel"
    ))
    .with_method("DELETE")
}

fn get_last_cpanel_config(server_number: u32) -> UnauthenticatedRequest<Single<ActiveCpanelConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/cpanel/last"
    ))
}

impl<Client: AsyncHttpClient> AsyncRobot<Client> {
    /// Retrieve a [`Server`](crate::api::server::Server)'s [`ActiveCpanelConfig`]
    /// configuration, or a list of available distributions and languages,
    /// if the Cpanel installation system is not currently active.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::boot::{Cpanel, ActiveCpanelConfig, AvailableCpanelConfig};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// match robot.get_cpanel_config(1234567).await.unwrap() {
    ///     Cpanel::Active(ActiveCpanelConfig { distribution, .. }) => {
    ///         println!("currently active cpanel distribution is: {distribution}");
    ///         // e.g.: currently active cpanel distribution is: CentOS-Stream
    ///     },
    ///     Cpanel::Available(AvailableCpanelConfig { distributions, .. }) => {
    ///         println!("available cpanel distributions are: {}", distributions.join(", "))
    ///         // e.g.: available cpanel distributions are: CentOS-Stream, ...
    ///     }
    /// }
    /// # }
    /// ```
    pub async fn get_cpanel_config(&self, server_number: u32) -> Result<Cpanel, Error> {
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
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_last_cpanel_config(1234567).await.unwrap();
    /// # }
    /// ```
    pub async fn get_last_cpanel_config(
        &self,
        server_number: u32,
    ) -> Result<ActiveCpanelConfig, Error> {
        Ok(self.go(get_last_cpanel_config(server_number)).await?.0)
    }

    /// Enable a linux installation configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::boot::{Cpanel, CpanelConfig};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_cpanel_config(1234567, CpanelConfig {
    ///     distribution: "CentOS-Stream".to_string(),
    ///     language: "en_US".to_string(),
    ///     hostname: "cpanel.example.com".to_string(),
    /// }).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_cpanel_config(
        &self,
        server_number: u32,
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
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_cpanel_config(1234567).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_cpanel_config(
        &self,
        server_number: u32,
    ) -> Result<AvailableCpanelConfig, Error> {
        Ok(self.go(disable_cpanel_config(server_number)).await?.0)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CpanelConfig {
    /// Distribution for the Cpanel installation.
    #[serde(rename = "dist")]
    pub distribution: String,

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
    pub distributions: Vec<String>,

    /// Available languages for the Cpanel installation.
    #[serde(rename = "lang")]
    pub languages: Vec<String>,
}

/// Currently active Cpanel configuration.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ActiveCpanelConfig {
    /// Distribution selected in currently active Cpanel installation.
    #[serde(rename = "dist")]
    pub distribution: String,

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
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Cpanel {
    /// Currently active Cpanel configuration.
    Active(ActiveCpanelConfig),
    /// Describes available Cpanel configuration options.
    Available(AvailableCpanelConfig),
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use tracing::info;
    use tracing_test::traced_test;

    use super::{Cpanel, CpanelConfig};

    #[tokio::test]
    #[traced_test]
    #[serial("boot-configuration")]
    async fn test_get_cpanel_configuration() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.first() {
            // Fetch the complete server object, so we can get check
            // if the Windows system is available for this server.
            let Some(availability) = robot.get_server(server.id).await.unwrap().availability else {
                return;
            };

            if availability.cpanel {
                let config = robot.get_cpanel_config(server.id).await.unwrap();
                info!("{config:#?}");
            }
        }
    }

    #[tokio::test]
    #[ignore = "enabling the Cpanel installation system is expensive, even if the system is never activated."]
    #[traced_test]
    #[serial("boot-configuration")]
    async fn test_enable_disable_cpanel() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.first() {
            let mut activated_config = robot
                .enable_cpanel_config(
                    server.id,
                    CpanelConfig {
                        distribution: "CentOS Stream".to_string(),
                        language: "en_US".to_string(),
                        hostname: "cpanel.example.com".to_string(),
                    },
                )
                .await
                .unwrap();

            let config = robot.get_cpanel_config(server.id).await.unwrap();
            info!("{config:#?}");

            assert_eq!(Cpanel::Active(activated_config.clone()), config);

            robot.disable_cpanel_config(server.id).await.unwrap();

            assert!(matches!(
                robot.get_cpanel_config(server.id).await.unwrap(),
                Cpanel::Available(_)
            ));

            // We null out the password so we can compare to the latest
            // config, since the latest does not include passwords.
            activated_config.password = None;

            assert_eq!(
                robot.get_last_cpanel_config(server.id).await.unwrap(),
                activated_config
            );
        }
    }

    #[tokio::test]
    #[traced_test]
    #[serial("boot-configuration")]
    async fn test_last_cpanel_config() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.first() {
            let Some(availability) = robot.get_server(server.id).await.unwrap().availability else {
                return;
            };

            if availability.cpanel {
                let last_config = robot.get_last_cpanel_config(server.id).await.unwrap();
                info!("{last_config:#?}");
            }
        }
    }
}
