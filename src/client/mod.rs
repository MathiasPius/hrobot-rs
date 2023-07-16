#[cfg(feature = "hyper-client")]
mod hyper;

#[cfg(feature = "async")]
mod r#async {
    use async_trait::async_trait;
    use serde::de::DeserializeOwned;
    use tracing::trace;

    use crate::{
        api::{AuthenticatedRequest, Credentials, UnauthenticatedRequest},
        error::{ApiResult, Error},
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
    /// # use hrobot::error::Error;
    /// # use hrobot::api::AuthenticatedRequest;
    /// # #[async_trait::async_trait]
    /// pub trait AsyncHttpClient {
    ///     async fn send_request<Response>(
    ///         &self,
    ///         request: AuthenticatedRequest<Response>,
    ///     ) -> Result<Response, Error>
    ///     where
    ///         Response: Send + 'static;
    /// }
    /// ```
    #[async_trait]
    pub trait AsyncHttpClient {
        /// Send an [`AuthenticatedRequest`] and return the deserialized
        /// `Response` or an [`Error`].
        ///
        /// Translating the [`AuthenticatedRequest`] and transmitting it
        /// through the underlying client is the responsibility of the
        /// implementor of this method.
        async fn send_request<Response>(
            &self,
            request: AuthenticatedRequest<Response>,
        ) -> Result<Vec<u8>, Error>
        where
            Response: Send + 'static;
    }

    /// Easy to use wrapper around an [`AsyncHttpClient`] implementation.
    ///
    /// Handles authentication and exposes the Hetzner Robot API functionality
    /// with a simple interface.
    ///
    /// In most cases, this is the struct you want to construct once
    /// and then use everywhere you want to interact with the API.
    ///
    /// # Client Requirements
    /// `Client` type is required to implement the [`AsyncHttpClient`].
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
    /// # std::env::set_var("HROBOT_USERNAME", "username");
    /// # std::env::set_var("HROBOT_PASSWORD", "password");
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

    impl<Client: AsyncHttpClient> AsyncRobot<Client> {
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
        pub(crate) async fn go<Response: DeserializeOwned + Send + 'static>(
            &self,
            request: UnauthenticatedRequest<Response>,
        ) -> Result<Response, Error> {
            trace!("{request:?}");

            let authenticated_request = request.authenticate(&self.credentials);

            let body = self.client.send_request(authenticated_request).await?;

            let stringified = String::from_utf8_lossy(&body);
            trace!("response body: {stringified}");

            serde_json::from_str::<ApiResult<Response>>(&stringified)?.into()
        }
    }
}

#[cfg(feature = "async")]
pub use r#async::*;
