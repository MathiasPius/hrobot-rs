use crate::client::{AsyncHttpClient, AsyncRobot};
use crate::{
    api::{keys::KeyReference, wrapper::Single, UnauthenticatedRequest},
    error::Error,
};
use serde::{Deserialize, Serialize};

fn get_linux_config(server_number: u32) -> UnauthenticatedRequest<Single<Linux>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/linux"
    ))
}

fn enable_linux_config(
    server_number: u32,
    linux: LinuxConfig,
) -> Result<UnauthenticatedRequest<Single<ActiveLinuxConfig>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/linux"
    ))
    .with_method("POST")
    .with_body(linux)
}

fn disable_linux_config(
    server_number: u32,
) -> UnauthenticatedRequest<Single<AvailableLinuxConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/linux"
    ))
    .with_method("DELETE")
}

fn get_last_linux_config(server_number: u32) -> UnauthenticatedRequest<Single<ActiveLinuxConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/linux/last"
    ))
}

impl<Client: AsyncHttpClient> AsyncRobot<Client> {
    /// Retrieve a [`Server`](crate::api::server::Server)'s [`ActiveLinuxConfig`]
    /// configuration, or a list of available operating systems, if the linux
    /// installation system is not currently active.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::boot::{Linux, ActiveLinuxConfig, AvailableLinuxConfig};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// match robot.get_linux_config(1234567).await.unwrap() {
    ///     Linux::Active(ActiveLinuxConfig { distribution, .. }) => {
    ///         println!("currently active linux distribution is: {distribution}");
    ///         // e.g.: currently active linux distribution is: Arch Linux latest minimal
    ///     },
    ///     Linux::Available(AvailableLinuxConfig { distributions, .. }) => {
    ///         println!("available linux distributions are: {}", distributions.join(", "))
    ///         // e.g.: available linux distributions are: Alma Linux 8.7, ...
    ///     }
    /// }
    /// # }
    /// ```
    pub async fn get_linux_config(&self, server_number: u32) -> Result<Linux, Error> {
        Ok(self.go(get_linux_config(server_number)).await?.0)
    }

    /// Get the last [`ActiveLinuxConfig`].
    ///
    /// This is the last configuration that was active on the server,
    /// not the *currently* active configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_last_linux_config(1234567).await.unwrap();
    /// # }
    /// ```
    pub async fn get_last_linux_config(
        &self,
        server_number: u32,
    ) -> Result<ActiveLinuxConfig, Error> {
        Ok(self.go(get_last_linux_config(server_number)).await?.0)
    }

    /// Enable a linux installation configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::boot::{Linux, LinuxConfig, Keyboard};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_linux_config(1234567, LinuxConfig {
    ///     distribution: "Arch Linux latest minimal".to_string(),
    ///     authorized_keys: vec!["d7:34:1c:8c:4e:20:e0:1f:07:66:45:d9:97:22:ec:07".to_string()],
    ///     language: "en".to_string(),
    /// }).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_linux_config(
        &self,
        server_number: u32,
        config: LinuxConfig,
    ) -> Result<ActiveLinuxConfig, Error> {
        Ok(self
            .go(enable_linux_config(server_number, config)?)
            .await?
            .0)
    }

    /// Disable the active linux installation configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_linux_config(1234567).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_linux_config(
        &self,
        server_number: u32,
    ) -> Result<AvailableLinuxConfig, Error> {
        Ok(self.go(disable_linux_config(server_number)).await?.0)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LinuxConfig {
    /// Distribution to install.
    #[serde(rename = "dist")]
    pub distribution: String,

    /// Language to use for the installation.
    #[serde(rename = "lang")]
    pub language: String,

    /// Authorized keys to add to the root user on
    /// the installed distribution.
    ///
    /// If not set, a root password will instead be
    /// configured, and returned as part of the reponse
    /// when activating the system.
    #[serde(rename = "authorized_key")]
    pub authorized_keys: Vec<String>,
}

/// Active Linux installation configuration.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ActiveLinuxConfig {
    /// Distribution to be installed.
    #[serde(rename = "dist")]
    pub distribution: String,

    /// Language of the distribution to be installed.
    #[serde(rename = "lang")]
    pub language: String,

    /// Root password to be set for the new distribution.
    ///
    /// Password is not configured, if an SSH key was provided
    /// when activating the Linux installation configuration.
    pub password: Option<String>,

    /// Keys to be added as authorized keys for the root user
    /// after installation has completed.
    #[serde(
        rename = "authorized_key",
        deserialize_with = "crate::api::wrapper::deserialize_inner_vec"
    )]
    pub authorized_keys: Vec<KeyReference>,

    /// Host keys to be installed with the new distribution.
    #[serde(rename = "host_key")]
    pub host_keys: Vec<String>,
}

/// Describes the Linux distributions and languages
/// available for installation.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AvailableLinuxConfig {
    /// Available linux distributions for installation.
    #[serde(rename = "dist")]
    pub distributions: Vec<String>,

    /// Available languages for installation.
    #[serde(rename = "lang")]
    pub languages: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Linux {
    /// Linux installation config is active.
    Active(ActiveLinuxConfig),
    /// Linux installation config is not active,
    /// these are the available distributions and languages.
    Available(AvailableLinuxConfig),
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use tracing::info;
    use tracing_test::traced_test;

    use super::{Linux, LinuxConfig};

    #[tokio::test]
    #[traced_test]
    #[serial("boot-configuration")]
    async fn test_get_linux_configuration() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.first() {
            let config = robot.get_linux_config(server.id).await.unwrap();
            info!("{config:#?}");
        }
    }

    #[tokio::test]
    #[ignore = "unexpected failure might leave the rescue system enabled."]
    #[traced_test]
    #[serial("boot-configuration")]
    async fn test_enable_disable_linux() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.first() {
            let mut activated_config = robot
                .enable_linux_config(
                    server.id,
                    LinuxConfig {
                        distribution: "Arch Linux latest minimal".to_string(),
                        language: "en".to_string(),
                        authorized_keys: vec![],
                    },
                )
                .await
                .unwrap();

            let config = robot.get_linux_config(server.id).await.unwrap();
            info!("{config:#?}");

            assert_eq!(Linux::Active(activated_config.clone()), config);

            robot.disable_linux_config(server.id).await.unwrap();

            assert!(matches!(
                robot.get_linux_config(server.id).await.unwrap(),
                Linux::Available(_)
            ));

            // We null out the password so we can compare to the latest
            // config, since the latest does not include passwords.
            activated_config.password = None;

            assert_eq!(
                robot.get_last_linux_config(server.id).await.unwrap(),
                activated_config
            );
        }
    }
}
