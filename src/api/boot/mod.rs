mod models;

use crate::{error::Error, AsyncHttpClient, AsyncRobot};

pub use models::{ActiveRescueConfig, AvailableRescueConfig, Keyboard, Rescue, RescueConfig};

use super::{wrapper::Single, UnauthenticatedRequest};

fn get_rescue_configuration(server_number: u32) -> UnauthenticatedRequest<Single<Rescue>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/rescue"
    ))
}

fn enable_rescue_configuration(
    server_number: u32,
    rescue: RescueConfig,
) -> Result<UnauthenticatedRequest<Single<ActiveRescueConfig>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/rescue"
    ))
    .with_method("POST")
    .with_body(rescue)
}

fn disable_rescue_configuration(
    server_number: u32,
) -> UnauthenticatedRequest<Single<AvailableRescueConfig>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/boot/{server_number}/rescue"
    ))
    .with_method("DELETE")
}

impl<Client: AsyncHttpClient> AsyncRobot<Client> {
    /// Retrieve a [`Server`]'s [`ActiveRescue`] configuration,
    /// or a list of available operating systems, if the rescue
    /// system is not currently active.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::boot::{Rescue, ActiveRescueConfig, AvailableRescueConfig};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// match robot.get_rescue_configuration(1234567).await.unwrap() {
    ///     Rescue::Active(ActiveRescueConfig { operating_system, .. }) => {
    ///         println!("currently active rescue system is: {operating_system}");
    ///         // e.g.: currently active rescue system is: vkvm
    ///     },
    ///     Rescue::Available(AvailableRescueConfig { operating_systems, .. }) => {
    ///         println!("available rescue systems are: {}", operating_systems.join(", "))
    ///         // e.g.: available rescue systems are: linux, linuxold, vkvm
    ///     }
    /// }
    /// # }
    /// ```
    pub async fn get_rescue_configuration(&self, server_number: u32) -> Result<Rescue, Error> {
        Ok(self.go(get_rescue_configuration(server_number)).await?.0)
    }

    /// Enable a rescue configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::boot::{Rescue, RescueConfig, Keyboard};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_rescue_configuration(1234567, RescueConfig {
    ///     operating_system: "vkvm".to_string(),
    ///     authorized_keys: vec!["d7:34:1c:8c:4e:20:e0:1f:07:66:45:d9:97:22:ec:07".to_string()],
    ///     keyboard: Keyboard::German,
    /// }).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_rescue_configuration(
        &self,
        server_number: u32,
        config: RescueConfig,
    ) -> Result<ActiveRescueConfig, Error> {
        Ok(self
            .go(enable_rescue_configuration(server_number, config)?)
            .await?
            .0)
    }

    /// Disable the active rescue configuration.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_rescue_configuration(1234567).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_rescue_configuration(
        &self,
        server_number: u32,
    ) -> Result<AvailableRescueConfig, Error> {
        Ok(self
            .go(disable_rescue_configuration(server_number))
            .await?
            .0)
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use tracing::info;
    use tracing_test::traced_test;

    use crate::api::boot::{Rescue, RescueConfig};

    #[tokio::test]
    #[traced_test]
    #[serial("boot-configuration")]
    async fn test_get_rescue_configuration() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.first() {
            let config = robot.get_rescue_configuration(server.id).await.unwrap();
            info!("{config:#?}");
        }
    }

    #[tokio::test]
    #[ignore = "unexpected failure might leave the rescue system enabled."]
    #[traced_test]
    #[serial("boot-configuration")]
    async fn test_enable_disable_vkm() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.first() {
            let activated_config = robot
                .enable_rescue_configuration(
                    server.id,
                    RescueConfig {
                        operating_system: "vkvm".to_string(),
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

            let config = robot.get_rescue_configuration(server.id).await.unwrap();
            info!("{config:#?}");

            assert_eq!(Rescue::Active(activated_config), config);

            robot.disable_rescue_configuration(server.id).await.unwrap();

            assert!(matches!(
                robot.get_rescue_configuration(server.id).await.unwrap(),
                Rescue::Available(_)
            ));
        }
    }
}
