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

    /// Implemented by asynchronous http clients, so they can be
    /// used with [`AsyncRobot`](AsyncRobot)
    ///
    /// The signature looks crazier than it is, because of the need
    /// for [`async_trait`](mod@async_trait),
    /// which will also be necessary when implementing it.
    ///
    /// The actual signature when using `async_trait` is:
    ///
    /// ```rust
    /// pub trait AsyncClient {
    ///     async fn send_request<Response>(
    ///         &self,
    ///         request: AuthenticatedRequest<Response>,
    ///     ) -> Result<Response, Error>
    ///     where
    ///         Response: DeserializeOwned + Send + 'static;
    /// }
    /// ```
    /// See the example below for an example implementation.
    /// # Example
    /// Implementation for [`hyper::Client`].
    ///
    /// Note: this implementation is included by when the
    /// `hyper-client` feature is enabled, which it is by default.
    /// ```rust
    /// # use async_trait::async_trait;
    /// #[async_trait]
    /// impl<C> AsyncClient for hyper::Client<C, Body>
    /// where
    ///     C: Connect + Clone + Send + Sync + 'static,
    /// {
    ///     async fn send_request<Response>(
    ///         &self,
    ///         request: AuthenticatedRequest<Response>,
    ///     ) -> Result<Response, Error>
    ///     where
    ///         Response: DeserializeOwned + Send + 'static,
    ///     {
    ///         // convert the request from an AuthenticatedRequest<Response>
    ///         // into a `hyper::Request`.
    ///         let request = request.try_into()
    ///             .map_err(Error::transport)?;
    ///
    ///         // send request and attempt wait for the response
    ///         let response = self.request(request).await
    ///             .map_err(Error::transport)?;
    ///
    ///         // get the response body
    ///         let body = hyper::body::to_bytes(response.into_body())
    ///             .await
    ///             .map_err(Error::transport)?;
    ///
    ///         // deserialize the response body into an `ApiResult<T>` containing
    ///         // either the expected `Response` type, or an `ApiError`.
    ///         serde_json::from_slice::<ApiResult<Response>>(&body)?.into()
    ///    }
    /// }
    /// ```
    #[async_trait]
    pub trait AsyncClient {
        /// Send an [`AuthenticatedRequest`] and return the deserialized
        /// `Response` or an [`Error`].
        ///
        /// Translating the [`AuthenticatedRequest`] and transmitting it
        /// through the underlying client is the responsibility of the
        /// implementor of this method.
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
    ///
    /// # Client Requirements
    /// `Client` type is required to implement the [`AsyncClient`].
    ///
    /// If the default feature `hyper-client` is enabled, then this
    /// crate implements it for [`hyper::Client`], and defines a default
    /// constructor for [`AsyncRobot<hyper::Client>`]
    ///
    /// See example below.
    ///
    /// # Example
    /// With the default `hyper-client` feature, you can construct
    /// an [`AsyncRobot`] using the `HROBOT_USERNAME` and `HROBOT_PASSWORD`
    /// environment variables with the [`AsyncRobot::default()`] function:
    /// ```rust
    /// # #[cfg(feature = "hyper-client")]
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// # }
    /// ```
    /// This uses [`hyper::Client`] and [`hyper_rustls`] to construct
    /// an HTTPS-enabled client, using credentials from the environment.
    pub struct AsyncRobot<Client> {
        credentials: Credentials,
        client: Client,
    }

    // Instead of requiring [`Debug`](std::fmt::Debug) be implemented
    // for all possible future clients, we just use whatever typename
    // for the client instead.
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
        /// `HROBOT_USERNAME` and `HROBOT_PASSWORD` for credentials,
        /// and the given client.
        ///
        /// # Example
        /// Construct an [`AsyncRobot`] using a [`hyper::Client`] and [`hyper_rustls`].
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

        /// Construct a new [`AsyncRobot`], using the given client, username and password.
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
        /// Print the ids and names of all servers accessible by our credentials.
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

        /// Withdraw a server cancellation.
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

        /// Withdraw a server order.
        ///
        /// # Example
        /// ```rust,no_run
        /// # #[tokio::main]
        /// # async fn main() {
        /// let robot = hrobot::AsyncRobot::default();
        /// let status = robot.withdraw_server_order(1234567, Some("Accidental purchase.")).await.unwrap();
        /// assert!(status.cancelled);
        /// # }
        /// ```
        pub async fn withdraw_server_order(
            &self,
            server_number: u32,
            reason: Option<&str>,
        ) -> Result<Cancellation, Error> {
            Ok(self
                .go(api::withdraw_server_order(server_number, reason)?)
                .await?
                .0)
        }
    }
}

#[cfg(feature = "async")]
pub use r#async::*;
