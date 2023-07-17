use serde::{Deserialize, Serialize};

use crate::{
    api::{wrapper::Single, UnauthenticatedRequest},
    error::Error,
    AsyncHttpClient, AsyncRobot,
};

fn get_vnc_config(server_number: u32) -> UnauthenticatedRequest<Single<Vnc>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/vnc"
    ))
}

fn enable_vnc_config(
    server_number: u32,
    config: VncConfig,
) -> Result<UnauthenticatedRequest<Single<ActiveVncConfig>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/vnc"
    ))
    .with_method("POST")
    .with_body(config)
}

fn disable_vnc_config(server_number: u32) -> UnauthenticatedRequest<Single<AvailableVncConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/vnc"
    ))
    .with_method("DELETE")
}

fn get_last_vnc_config(server_number: u32) -> UnauthenticatedRequest<Single<ActiveVncConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/vnc/last"
    ))
}

impl<Client: AsyncHttpClient> AsyncRobot<Client> {
    /// Retrieve a [`Server`](crate::api::server::Server)'s [`ActiveVncConfig`]
    /// configuration, or a list of available distributions and languages,
    /// if the vnc installation system is not currently active.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::boot::{Vnc, ActiveVncConfig, AvailableVncConfig};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// match robot.get_vnc_config(1234567).await.unwrap() {
    ///     Vnc::Active(ActiveVncConfig { distribution, .. }) => {
    ///         println!("currently active vnc distribution is: {distribution}");
    ///         // e.g.: currently active vnc distribution is: Fedora-37
    ///     },
    ///     Vnc::Available(AvailableVncConfig { distributions, .. }) => {
    ///         println!("available vnc distributions are: {}", distributions.join(", "))
    ///         // e.g.: available vnc distributions are: Fedora-37, ...
    ///     }
    /// }
    /// # }
    /// ```
    pub async fn get_vnc_config(&self, server_number: u32) -> Result<Vnc, Error> {
        Ok(self.go(get_vnc_config(server_number)).await?.0)
    }

    /// Get the last [`ActiveVncConfig`].
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
    /// robot.get_last_vnc_config(1234567).await.unwrap();
    /// # }
    /// ```
    pub async fn get_last_vnc_config(&self, server_number: u32) -> Result<ActiveVncConfig, Error> {
        Ok(self.go(get_last_vnc_config(server_number)).await?.0)
    }

    /// Enable a VNC installation configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::boot::{Vnc, VncConfig};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_vnc_config(1234567, VncConfig {
    ///     distribution: "Fedora-37".to_string(),
    ///     language: "en_US".to_string(),
    /// }).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_vnc_config(
        &self,
        server_number: u32,
        config: VncConfig,
    ) -> Result<ActiveVncConfig, Error> {
        Ok(self.go(enable_vnc_config(server_number, config)?).await?.0)
    }

    /// Disable the active VNC installation configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_vnc_config(1234567).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_vnc_config(
        &self,
        server_number: u32,
    ) -> Result<AvailableVncConfig, Error> {
        Ok(self.go(disable_vnc_config(server_number)).await?.0)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct VncConfig {
    /// Distribution for the VNC installation.
    #[serde(rename = "dist")]
    pub distribution: String,

    /// Language of the distribution
    #[serde(rename = "lang")]
    pub language: String,
}

/// Describes available VNC configuration options.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AvailableVncConfig {
    /// Available distributions for VNC installation.
    #[serde(rename = "dist")]
    pub distributions: Vec<String>,

    /// Available languages for the VNC installation.
    #[serde(rename = "lang")]
    pub languages: Vec<String>,
}

/// Currently active VNC configuration.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ActiveVncConfig {
    /// Distribution selected in currently active VNC installation.
    #[serde(rename = "dist")]
    pub distribution: String,

    /// Language selected in currently active VNC installation.
    #[serde(rename = "lang")]
    pub language: String,

    /// Password for the VNC installation.
    pub password: Option<String>,
}

/// Describes either the active or available VNC installation configurations.
///
/// If a VNC configuration system is active, it ([`ActiveVncConfig`]) will be returned,
/// otherwise a struct ([`AvailableVncConfig`]) representing the available VNC distributions
/// and languages are returned.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Vnc {
    /// Currently active VNC configuration.
    Active(ActiveVncConfig),
    /// Describes available VNC configuration options.
    Available(AvailableVncConfig),
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use tracing::info;
    use tracing_test::traced_test;

    use super::{Vnc, VncConfig};

    #[tokio::test]
    #[traced_test]
    #[serial("boot-configuration")]
    async fn test_get_vnc_configuration() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.first() {
            let config = robot.get_vnc_config(server.id).await.unwrap();
            info!("{config:#?}");
        }
    }

    #[tokio::test]
    #[ignore = "unexpected failure might leave the vnc installation system enabled."]
    #[traced_test]
    #[serial("boot-configuration")]
    async fn test_enable_disable_vnc() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.first() {
            let mut activated_config = robot
                .enable_vnc_config(
                    server.id,
                    VncConfig {
                        distribution: "Fedora-37".to_string(),
                        language: "en_US".to_string(),
                    },
                )
                .await
                .unwrap();

            let config = robot.get_vnc_config(server.id).await.unwrap();
            info!("{config:#?}");

            assert_eq!(Vnc::Active(activated_config.clone()), config);

            robot.disable_vnc_config(server.id).await.unwrap();

            assert!(matches!(
                robot.get_vnc_config(server.id).await.unwrap(),
                Vnc::Available(_)
            ));

            // We null out the password so we can compare to the latest
            // config, since the latest does not include passwords.
            activated_config.password = None;

            assert_eq!(
                robot.get_last_vnc_config(server.id).await.unwrap(),
                activated_config
            );
        }
    }

    #[tokio::test]
    #[traced_test]
    #[serial("boot-configuration")]
    async fn test_last_vnc_config() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.first() {
            let last_config = robot.get_last_vnc_config(server.id).await.unwrap();

            println!("{last_config:#?}");
        }
    }
}
