#[cfg(feature = "hyper-client")]
mod hyper;

#[cfg(feature = "async")]
mod r#async {
    use async_trait::async_trait;
    use serde::de::DeserializeOwned;

    use crate::{
        api::{self, AuthenticatedRequest, Credentials, UnauthenticatedRequest},
        data::Server,
        error::Error,
    };

    /// Implemented by asynchronous http clients, so they can be used with [`AsyncRobot`](AsyncRobot)
    #[async_trait]
    pub trait AsyncClient {
        async fn send_request<Response>(
            &self,
            request: AuthenticatedRequest<Response>,
        ) -> Result<Response, Error>
        where
            Response: DeserializeOwned + Send + 'static;
    }

    /// Easy to use wrapper around an [`AsyncClient`] implementation.
    ///
    /// Handles authentication and exposes the Hetzner Robot API functionality
    /// with a simple interface.
    ///
    /// In most cases, this is the struct you want to construct once
    /// and then use everywhere you want to interact with the API.
    pub struct AsyncRobot<Client> {
        credentials: Credentials,
        client: Client,
    }

    impl<Client: AsyncClient> AsyncRobot<Client> {
        pub fn from_env(client: Client) -> Result<Self, std::env::VarError> {
            Ok(Self::new(
                client,
                &std::env::var("HROBOT_USERNAME")?,
                &std::env::var("HROBOT_PASSWORD")?,
            ))
        }

        pub fn new(client: Client, username: &str, password: &str) -> Self {
            AsyncRobot {
                credentials: Credentials::new(username, password),
                client,
            }
        }

        /// Shorthand for authenticating and sending the request.
        async fn go<Response: DeserializeOwned + Send + 'static>(
            &self,
            request: UnauthenticatedRequest<Response>,
        ) -> Result<Response, Error> {
            let authenticated_request = request.authenticate(&self.credentials);
            self.client.send_request(authenticated_request).await
        }

        pub async fn list_servers(&self) -> Result<Vec<Server>, Error> {
            Ok(self.go(api::list_servers()).await?.0)
        }

        pub async fn get_server(&self, server_number: u32) -> Result<Server, Error> {
            Ok(self.go(api::get_server(server_number)).await?.0)
        }

        pub async fn rename_server(&self, server_number: u32, name: &str) -> Result<Server, Error> {
            Ok(self.go(api::rename_server(server_number, name)?).await?.0)
        }
    }
}

#[cfg(feature = "async")]
pub use r#async::*;
