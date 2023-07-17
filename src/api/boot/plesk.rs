use serde::{Deserialize, Serialize};

use crate::{
    api::{wrapper::Single, UnauthenticatedRequest},
    error::Error,
    AsyncHttpClient, AsyncRobot,
};

fn get_plesk_config(server_number: u32) -> UnauthenticatedRequest<Single<Plesk>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/plesk"
    ))
}

fn enable_plesk_config(
    server_number: u32,
    config: PleskConfig,
) -> Result<UnauthenticatedRequest<Single<ActivePleskConfig>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/plesk"
    ))
    .with_method("POST")
    .with_body(config)
}

fn disable_plesk_config(
    server_number: u32,
) -> UnauthenticatedRequest<Single<AvailablePleskConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/plesk"
    ))
    .with_method("DELETE")
}

fn get_last_plesk_config(server_number: u32) -> UnauthenticatedRequest<Single<ActivePleskConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/plesk/last"
    ))
}

impl<Client: AsyncHttpClient> AsyncRobot<Client> {
    /// Retrieve a [`Server`](crate::api::server::Server)'s [`ActivePleskConfig`]
    /// configuration, or a list of available distributions and languages,
    /// if the Plesk installation system is not currently active.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::boot::{Plesk, ActivePleskConfig, AvailablePleskConfig};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// match robot.get_plesk_config(1234567).await.unwrap() {
    ///     Plesk::Active(ActivePleskConfig { distribution, .. }) => {
    ///         println!("currently active plesk distribution is: {distribution}");
    ///         // e.g.: currently active plesk distribution is: CentOS-Stream
    ///     },
    ///     Plesk::Available(AvailablePleskConfig { distributions, .. }) => {
    ///         println!("available plesk distributions are: {}", distributions.join(", "))
    ///         // e.g.: available plesk distributions are: CentOS-Stream, ...
    ///     }
    /// }
    /// # }
    /// ```
    pub async fn get_plesk_config(&self, server_number: u32) -> Result<Plesk, Error> {
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
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_last_plesk_config(1234567).await.unwrap();
    /// # }
    /// ```
    pub async fn get_last_plesk_config(
        &self,
        server_number: u32,
    ) -> Result<ActivePleskConfig, Error> {
        Ok(self.go(get_last_plesk_config(server_number)).await?.0)
    }

    /// Enable a linux installation configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::boot::{Plesk, PleskConfig};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_plesk_config(1234567, PleskConfig {
    ///     distribution: "CentOS-Stream".to_string(),
    ///     language: "en_US".to_string(),
    ///     hostname: "plesk.example.com".to_string(),
    /// }).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_plesk_config(
        &self,
        server_number: u32,
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
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_plesk_config(1234567).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_plesk_config(
        &self,
        server_number: u32,
    ) -> Result<AvailablePleskConfig, Error> {
        Ok(self.go(disable_plesk_config(server_number)).await?.0)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PleskConfig {
    /// Distribution for the Plesk installation.
    #[serde(rename = "dist")]
    pub distribution: String,

    /// Hostname for the Plesk installation.
    pub hostname: String,

    /// Language of the distribution
    #[serde(rename = "lang")]
    pub language: String,
}

/// Describes available Plesk configuration options.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AvailablePleskConfig {
    /// Available distributions for Plesk installation.
    #[serde(rename = "dist")]
    pub distributions: Vec<String>,

    /// Available languages for the Plesk installation.
    #[serde(rename = "lang")]
    pub languages: Vec<String>,
}

/// Currently active Plesk configuration.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ActivePleskConfig {
    /// Distribution selected in currently active Plesk installation.
    #[serde(rename = "dist")]
    pub distribution: String,

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
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Plesk {
    /// Currently active Plesk configuration.
    Active(ActivePleskConfig),
    /// Describes available Plesk configuration options.
    Available(AvailablePleskConfig),
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use tracing::info;
    use tracing_test::traced_test;

    use super::{Plesk, PleskConfig};

    #[tokio::test]
    #[traced_test]
    #[serial("boot-configuration")]
    async fn test_get_plesk_configuration() {
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

            if availability.plesk {
                let config = robot.get_plesk_config(server.id).await.unwrap();
                info!("{config:#?}");
            }
        }
    }

    #[tokio::test]
    #[ignore = "enabling the Plesk installation system is expensive, even if the system is never activated."]
    #[traced_test]
    #[serial("boot-configuration")]
    async fn test_enable_disable_plesk() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.first() {
            let mut activated_config = robot
                .enable_plesk_config(
                    server.id,
                    PleskConfig {
                        distribution: "CentOS Stream".to_string(),
                        language: "en_US".to_string(),
                        hostname: "plesk.example.com".to_string(),
                    },
                )
                .await
                .unwrap();

            let config = robot.get_plesk_config(server.id).await.unwrap();
            info!("{config:#?}");

            assert_eq!(Plesk::Active(activated_config.clone()), config);

            robot.disable_plesk_config(server.id).await.unwrap();

            assert!(matches!(
                robot.get_plesk_config(server.id).await.unwrap(),
                Plesk::Available(_)
            ));

            // We null out the password so we can compare to the latest
            // config, since the latest does not include passwords.
            activated_config.password = None;

            assert_eq!(
                robot.get_last_plesk_config(server.id).await.unwrap(),
                activated_config
            );
        }
    }

    #[tokio::test]
    #[traced_test]
    #[serial("boot-configuration")]
    async fn test_last_plesk_config() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.first() {
            let Some(availability) = robot.get_server(server.id).await.unwrap().availability else {
                return;
            };

            if availability.plesk {
                let last_config = robot.get_last_plesk_config(server.id).await.unwrap();
                info!("{last_config:#?}");
            }
        }
    }
}
