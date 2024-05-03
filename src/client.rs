mod r#async {
    use http_body_util::{BodyExt, Full};
    use hyper::body::Bytes;
    use hyper_rustls::HttpsConnector;
    use hyper_util::{
        client::legacy::{connect::HttpConnector, Client},
        rt::TokioExecutor,
    };
    use serde::de::DeserializeOwned;
    use tracing::trace;

    use crate::{
        api::{Credentials, UnauthenticatedRequest},
        error::{Error, MaybeTypedResponse},
    };

    /// Handles authentication and exposes the Hetzner Robot API functionality
    /// with a simple interface.
    ///
    /// # Example
    /// an [`AsyncRobot`] using the `HROBOT_USERNAME` and `HROBOT_PASSWORD`
    /// environment variables:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() {
    /// # std::env::set_var("HROBOT_USERNAME", "username");
    /// # std::env::set_var("HROBOT_PASSWORD", "password");
    /// let robot = hrobot::AsyncRobot::default();
    /// # }
    /// ```
    ///
    /// If you want to customize the [`hyper::Client`] see:
    /// * [`AsyncRobot::from_env`] if you still want to use the environment variables, or
    /// * [`AsyncRobot::new`] if you want to provide client and credentials yourself.
    ///
    #[derive(Debug, Clone)]
    pub struct AsyncRobot {
        credentials: Credentials,
        client: Client<HttpsConnector<HttpConnector>, Full<Bytes>>,
    }

    impl Default for AsyncRobot {
        fn default() -> Self {
            let https: HttpsConnector<HttpConnector> = hyper_rustls::HttpsConnectorBuilder::new()
                .with_webpki_roots()
                .https_only()
                .enable_http1()
                .build();
            let client = Client::builder(TokioExecutor::new()).build(https);

            Self::from_env(client).unwrap()
        }
    }

    impl AsyncRobot {
        /// Construct a new [`AsyncRobot`] using the environment variables
        /// `HROBOT_USERNAME` and `HROBOT_PASSWORD` for credentials,
        /// and the given client.
        ///
        /// # Example
        /// Construct an [`AsyncRobot`] using a [`hyper_util::client::legacy::Client`] and [`hyper_rustls`].
        /// ```rust
        /// # #[tokio::main]
        /// # async fn main() {
        /// let https = hyper_rustls::HttpsConnectorBuilder::new()
        ///     .with_webpki_roots()
        ///     .https_only()
        ///     .enable_http1()
        ///     .build();
        ///
        /// let client = hyper_util::client::legacy::Client::builder(
        ///     hyper_util::rt::TokioExecutor::new()
        /// ).build(https);
        ///
        /// let robot = hrobot::AsyncRobot::from_env(client);
        /// # }
        /// ```
        pub fn from_env(
            client: Client<HttpsConnector<HttpConnector>, Full<Bytes>>,
        ) -> Result<Self, std::env::VarError> {
            Ok(Self::new(
                client,
                &std::env::var("HROBOT_USERNAME")?,
                &std::env::var("HROBOT_PASSWORD")?,
            ))
        }

        /// Construct a new [`AsyncRobot`], using the given client, username and password.
        ///
        /// # Example
        /// Construct an [`AsyncRobot`] using a custom [`hyper_util::client::legacy::Client`].
        /// ```rust
        /// # #[tokio::main]
        /// # async fn main() {
        /// let https = hyper_rustls::HttpsConnectorBuilder::new()
        ///     .with_webpki_roots()
        ///     .https_only()
        ///     .enable_http1()
        ///     .build();
        ///
        /// let client = hyper_util::client::legacy::Client::builder(
        ///     hyper_util::rt::TokioExecutor::new()
        /// ).build(https);
        ///
        /// let robot = hrobot::AsyncRobot::new(client, "#ws+username", "p@ssw0rd");
        /// # }
        /// ```
        pub fn new(
            client: Client<HttpsConnector<HttpConnector>, Full<Bytes>>,
            username: &str,
            password: &str,
        ) -> Self {
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

            let body = match authenticated_request.body() {
                None => Full::default(),
                Some(value) => Full::from(value.to_owned()),
            };

            let request = hyper::Request::builder()
                .uri(authenticated_request.uri())
                .method(authenticated_request.method())
                .header(
                    "Authorization",
                    authenticated_request.authorization_header(),
                )
                .header("Content-Type", "application/x-www-form-urlencoded")
                .header("Accept", "application/json")
                .body(body)
                .map_err(Error::transport)?;

            let response = self
                .client
                .request(request)
                .await
                .map_err(Error::transport)?;

            let body = response
                .into_body()
                .collect()
                .await
                .map_err(Error::transport)?
                .to_bytes();

            let stringified = String::from_utf8_lossy(&body);
            trace!("response body: {stringified}");

            // We do explicit deserialization here, since some endpoints can return empty responses.
            //
            // I initialize used a #[derive(Serialize, Deserialize)] enum which encapsulated both success and
            // error states, but deserializing an untagged enum, even when the encapsulated "Ok"
            // result is just a unit type (), deserialization will fail on empty input.
            if let Ok(result) = serde_json::from_str::<Response>(&stringified) {
                Ok(result)
            } else {
                match serde_json::from_str::<MaybeTypedResponse>(&stringified) {
                    Ok(api_error) => Err(Error::Api(api_error.error.into())),
                    Err(serde) => Err(Error::Deserialization(serde)),
                }
            }
        }
    }
}

pub use r#async::*;
