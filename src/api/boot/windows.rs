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
    ///         println!("available windows installation distributions are: {}", distributions.join(", "))
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
    /// # use hrobot::api::boot::WindowsConfig;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_windows_config(ServerId(1234567), WindowsConfig {
    ///     distribution: "standard".to_string(),
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
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ActiveWindowsConfig {
    /// Active Windows installation distribution.
    #[serde(rename = "dist")]
    pub distribution: String,

    /// Active Windows installation language.
    #[serde(rename = "lang")]
    pub language: String,

    /// Administrator password for the currently active
    /// Windows installation configuration.
    pub password: Option<String>,
}

/// availble Windows installation configuration options.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AvailableWindowsConfig {
    /// Available Windows installation distributions.
    #[serde(rename = "dist")]
    pub distributions: Vec<String>,

    /// Available Windows installation languages.
    #[serde(rename = "lang")]
    pub language: Vec<String>,
}

/// Describes either the active or available Windows configurations.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
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
    pub distribution: String,

    /// Language to install.
    #[serde(rename = "lang")]
    pub language: String,
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
        async fn test_get_windows_configuration() {
            dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let servers = robot.list_servers().await.unwrap();
            info!("{servers:#?}");

            if let Some(server) = servers.first() {
                // Fetch the complete server object, so we can get check
                // if the Windows system is available for this server.
                let Some(availability) = robot.get_server(server.id).await.unwrap().availability
                else {
                    return;
                };

                if availability.windows {
                    let config = robot.get_windows_config(server.id).await.unwrap();
                    info!("{config:#?}");
                }
            }
        }

        #[tokio::test]
        #[traced_test]
        #[serial("boot-configuration")]
        async fn test_last_windows_config() {
            dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let servers = robot.list_servers().await.unwrap();
            info!("{servers:#?}");

            if let Some(server) = servers.first() {
                let Some(availability) = robot.get_server(server.id).await.unwrap().availability
                else {
                    return;
                };

                if availability.windows {
                    let last_config = robot.get_last_windows_config(server.id).await.unwrap();
                    info!("{last_config:#?}");
                }
            }
        }
    }

    #[cfg(feature = "disruptive-tests")]
    mod disruptive_tests {
        use serial_test::serial;
        use tracing::info;
        use tracing_test::traced_test;

        use crate::api::boot::{Windows, WindowsConfig};

        #[tokio::test]
        #[ignore = "enabling the Windows installation system is expensive, even if the system is never activated."]
        #[traced_test]
        #[serial("boot-configuration")]
        async fn test_enable_disable_windows() {
            dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let servers = robot.list_servers().await.unwrap();
            info!("{servers:#?}");

            if let Some(server) = servers.first() {
                let mut activated_config = robot
                    .enable_windows_config(
                        server.id,
                        WindowsConfig {
                            distribution: "standard".to_string(),
                            language: "en_US".to_string(),
                        },
                    )
                    .await
                    .unwrap();

                let config = robot.get_windows_config(server.id).await.unwrap();
                info!("{config:#?}");

                assert_eq!(Windows::Active(activated_config.clone()), config);

                robot.disable_windows_config(server.id).await.unwrap();

                assert!(matches!(
                    robot.get_windows_config(server.id).await.unwrap(),
                    Windows::Available(_)
                ));

                // We null out the password so we can compare to the latest
                // config, since the latest does not include passwords.
                activated_config.password = None;

                assert_eq!(
                    robot.get_last_windows_config(server.id).await.unwrap(),
                    activated_config
                );
            }
        }
    }
}
