#[cfg(feature = "hyper-client")]
mod hyper;

#[cfg(feature = "async")]
mod r#async {
    use async_trait::async_trait;
    use serde::de::DeserializeOwned;
    use tracing::trace;

    use crate::{
        api::{self, AuthenticatedRequest, Credentials, UnauthenticatedRequest},
        data::{Cancellation, Server},
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

    impl<Client> std::fmt::Debug for AsyncRobot<Client> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AsyncRobot")
                .field("credentials", &self.credentials)
                .field("client type", &std::any::type_name::<Client>())
                .finish()
        }
    }

    impl<Client: AsyncClient> AsyncRobot<Client> {
        /// Construct a new [`AsyncRobot`] using the environment variables
        /// `HROBOT_USERNAME` and `HROBOT_PASSWORD` for credentials.
        ///
        /// # Example
        /// Construct an [`AsyncRobot`] using a [`hyper::Client`].
        /// ```rust
        /// # #[cfg(feature = "hyper-client")]
        /// # #[tokio::main]
        /// # async fn main() {
        /// let https = hyper_rustls::HttpsConnectorBuilder::new()
        ///     .with_native_roots()
        ///     .https_only()
        ///     .enable_http1()
        ///     .build();
        ///
        /// let client = hyper::Client::builder().build(https);
        ///
        /// let robot = hrobot::AsyncRobot::from_env(client);
        /// # }
        /// ```
        pub fn from_env(client: Client) -> Result<Self, std::env::VarError> {
            Ok(Self::new(
                client,
                &std::env::var("HROBOT_USERNAME")?,
                &std::env::var("HROBOT_PASSWORD")?,
            ))
        }

        /// Construct a new [`AsyncRobot`].
        ///
        /// # Example
        /// Construct an [`AsyncRobot`] using a [`hyper::Client`].
        /// ```rust
        /// # #[cfg(feature = "hyper-client")]
        /// # #[tokio::main]
        /// # async fn main() {
        /// let https = hyper_rustls::HttpsConnectorBuilder::new()
        ///     .with_native_roots()
        ///     .https_only()
        ///     .enable_http1()
        ///     .build();
        ///
        /// let client = hyper::Client::builder().build(https);
        ///
        /// let robot = hrobot::AsyncRobot::new(client, "#ws+username", "p@ssw0rd");
        /// # }
        /// ```
        pub fn new(client: Client, username: &str, password: &str) -> Self {
            AsyncRobot {
                credentials: Credentials::new(username, password),
                client,
            }
        }

        /// Shorthand for authenticating and sending the request.

        #[tracing::instrument]
        async fn go<Response: DeserializeOwned + Send + 'static>(
            &self,
            request: UnauthenticatedRequest<Response>,
        ) -> Result<Response, Error> {
            trace!("{request:?}");

            let authenticated_request = request.authenticate(&self.credentials);

            self.client.send_request(authenticated_request).await
        }

        /// List all owned servers.
        ///
        /// # Example
        /// Print the IDs and Names of all our servers.
        /// ```rust,no_run
        /// # #[tokio::main]
        /// # async fn main() {
        /// # let robot = hrobot::AsyncRobot::default();
        /// for server in robot.list_servers().await.unwrap() {
        ///     println!("{}: {}", server.id, server.name);
        /// }
        /// # }
        /// ```
        pub async fn list_servers(&self) -> Result<Vec<Server>, Error> {
            Ok(self.go(api::list_servers()).await?.0)
        }

        /// Retrieve complete information about a specific [`Server`].
        ///
        /// # Example
        /// ```rust,no_run
        /// # #[tokio::main]
        /// # async fn main() {
        /// let robot = hrobot::AsyncRobot::default();
        /// let server = robot.get_server(1234567).await.unwrap();
        /// assert_eq!(server.id, 1234567);
        /// println!("Name: {}", server.name);
        /// # }
        /// ```
        pub async fn get_server(&self, server_number: u32) -> Result<Server, Error> {
            Ok(self.go(api::get_server(server_number)).await?.0)
        }

        /// Rename a server.
        ///
        /// # Example
        /// ```rust,no_run
        /// # #[tokio::main]
        /// # async fn main() {
        /// let robot = hrobot::AsyncRobot::default();
        /// robot.rename_server(1234567, "gibson").await.unwrap();
        /// # }
        /// ```
        pub async fn rename_server(&self, server_number: u32, name: &str) -> Result<Server, Error> {
            Ok(self.go(api::rename_server(server_number, name)?).await?.0)
        }

        /// Get the current cancellation status of a server.
        ///
        /// # Example
        /// ```rust,no_run
        /// # #[tokio::main]
        /// # async fn main() {
        /// let robot = hrobot::AsyncRobot::default();
        /// let status = robot.get_server_cancellation(1234567).await.unwrap();
        /// assert!(!status.cancelled);
        /// # }
        /// ```
        pub async fn get_server_cancellation(
            &self,
            server_number: u32,
        ) -> Result<Cancellation, Error> {
            Ok(self
                .go(api::get_server_cancellation(server_number))
                .await?
                .0)
        }

        /// Withdraw a server cancellation
        ///
        /// # Example
        /// ```rust,no_run
        /// # #[tokio::main]
        /// # async fn main() {
        /// let robot = hrobot::AsyncRobot::default();
        /// let status = robot.withdraw_server_cancellation(1234567).await.unwrap();
        /// assert!(!status.cancelled);
        /// # }
        /// ```
        pub async fn withdraw_server_cancellation(
            &self,
            server_number: u32,
        ) -> Result<Cancellation, Error> {
            Ok(self
                .go(api::withdraw_server_cancellation(server_number))
                .await?
                .0)
        }
    }
}

#[cfg(feature = "async")]
pub use r#async::*;
