use serde::Deserialize;

use crate::{
    error::{ApiError, Error},
    AsyncHttpClient, AsyncRobot,
};

use super::{wrapper::Single, UnauthenticatedRequest};

fn get_wake_on_lan(server_number: u32) -> UnauthenticatedRequest<Single<Wol>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/wol/{server_number}"
    ))
}

fn post_wake_on_lan(server_number: u32) -> UnauthenticatedRequest<Single<Wol>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/wol/{server_number}"
    ))
    .with_method("POST")
}

impl<Client: AsyncHttpClient> AsyncRobot<Client> {
    /// Check if Wake-on-LAN is available for the server.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// assert!(robot.is_wake_on_lan_available(1234567).await.unwrap());
    /// # }
    /// ```
    pub async fn is_wake_on_lan_available(&self, server_number: u32) -> Result<bool, Error> {
        let response = self.go(get_wake_on_lan(server_number)).await;

        match response {
            Ok(_) => Ok(true),
            Err(Error::Api(ApiError::WolNotAvailable { .. })) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Send a Wake-on-LAN packet to the specified server.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.trigger_wake_on_lan(1234567).await.unwrap();
    /// # }
    /// ```
    pub async fn trigger_wake_on_lan(&self, server_number: u32) -> Result<(), Error> {
        self.go(post_wake_on_lan(server_number)).await.map(|_| ())
    }
}

// The API endpoint returns a struct with information about the server,
// but we only care about the presence of a non-404 response.
#[derive(Debug, Deserialize)]
struct Wol {
    #[serde(rename = "server_number")]
    _server_number: u32,
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_wake_on_lan_available() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.first() {
            if robot.is_wake_on_lan_available(server.id).await.unwrap() {
                info!("{}: wake on lan is available", server.name);
            } else {
                info!("{}: wake on lan is NOT available", server.name);
            }
        }
    }
}
