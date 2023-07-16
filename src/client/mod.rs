#[cfg(feature = "hyper-client")]
mod hyper;

#[cfg(feature = "async")]
mod r#async {
    use async_trait::async_trait;
    use serde::de::DeserializeOwned;
    use tracing::trace;

    use crate::{
        api::{self, AuthenticatedRequest, Credentials, UnauthenticatedRequest},
        error::{ApiResult, Error},
        models::{
            Cancellation, Firewall, FirewallConfiguration, FirewallTemplate,
            FirewallTemplateConfiguration, FirewallTemplateReference, Server,
        },
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

        /// List all owned servers.
        ///
        /// # Example
        /// Print the ids and names of all servers accessible by our credentials.
        /// ```rust
        /// # #[tokio::main]
        /// # async fn main() {
        /// # dotenvy::dotenv().ok();
        /// let robot = hrobot::AsyncRobot::default();
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

        /// Retrieve a [`Server`]'s [`Firewall`].
        ///
        /// # Example
        /// ```rust,no_run
        /// # #[tokio::main]
        /// # async fn main() {
        /// let robot = hrobot::AsyncRobot::default();
        /// let firewall = robot.get_firewall(1234567).await.unwrap();
        /// println!("Ingress rule count: {}", firewall.rules.ingress.len());
        /// # }
        /// ```
        pub async fn get_firewall(&self, server_number: u32) -> Result<Firewall, Error> {
            Ok(self.go(api::get_firewall(server_number)).await?.0.into())
        }

        /// Replace a [`Server`]'s [`Firewall`] configuration.
        ///
        /// **Warning**: This replaces the entire firewall for
        /// both directions! If you don't define any ingress or
        /// egress rules, only the default-deny rule will apply!
        ///
        /// # Example
        /// ```rust,no_run
        /// # use std::net::Ipv4Addr;
        /// # use hrobot::models::{
        /// #     FirewallConfiguration, Rule, Rules, State, Ipv4Filter
        /// # };
        /// # #[tokio::main]
        /// # async fn main() {
        /// let robot = hrobot::AsyncRobot::default();
        ///
        /// let firewall = FirewallConfiguration {
        ///    status: State::Active,
        ///    filter_ipv6: false,
        ///    whitelist_hetzner_services: true,
        ///    rules: Rules {
        ///        ingress: vec![
        ///            Rule::accept("Allow from home").matching(
        ///                 Ipv4Filter::tcp(None)
        ///                     .from_ip(Ipv4Addr::new(123, 123, 123, 123))
        ///                     .to_port(27015..=27016)
        ///            )
        ///        ],
        ///        egress: vec![
        ///            Rule::accept("Allow all")
        ///        ]
        ///    },
        /// };
        ///
        /// robot.set_firewall_configuration(1234567, &firewall).await.unwrap();
        /// # }
        /// ```
        pub async fn set_firewall_configuration(
            &self,
            server_number: u32,
            firewall: &FirewallConfiguration,
        ) -> Result<Firewall, Error> {
            Ok(self
                .go(api::set_firewall_configuration(server_number, firewall)?)
                .await?
                .0
                .into())
        }

        /// Clear a [`Server`]s [`Firewall`] configuration.
        ///
        /// This reverts the server's firewall configuration to
        /// default Hetzner firewall, which has "Allow all" rules
        /// in both directions.
        ///  
        /// # Example
        /// ```rust,no_run
        /// # #[tokio::main]
        /// # async fn main() {
        /// let robot = hrobot::AsyncRobot::default();
        /// robot.delete_firewall(1234567).await.unwrap();
        /// # }
        /// ```
        pub async fn delete_firewall(&self, server_number: u32) -> Result<Firewall, Error> {
            Ok(self.go(api::delete_firewall(server_number)).await?.0.into())
        }

        /// List all firewall templates.
        ///
        /// This only returns a list of [`FirewallTemplateReference`],
        /// which do not include the complete firewall configuration.
        ///
        /// use [`AsyncRobot::get_firewall_template()`] with the returned
        /// template ID, if you want to get the configuration.
        ///
        /// # Example
        /// ```rust
        /// # #[tokio::main]
        /// # async fn main() {
        /// # dotenvy::dotenv().ok();
        /// let robot = hrobot::AsyncRobot::default();
        /// let templates = robot.list_firewall_templates().await.unwrap();
        /// # }
        /// ```
        pub async fn list_firewall_templates(
            &self,
        ) -> Result<Vec<FirewallTemplateReference>, Error> {
            Ok(self.go(api::list_firewall_templates()).await?.0.into())
        }

        /// Retrieve a complete [`FirewallTemplate`].
        ///
        /// This returns the entire template, including its rules.
        ///
        /// # Example
        /// ```rust,no_run
        /// # #[tokio::main]
        /// # async fn main() {
        /// let robot = hrobot::AsyncRobot::default();
        /// let template = robot.get_firewall_template(1234).await.unwrap();
        /// # }
        /// ```
        pub async fn get_firewall_template(
            &self,
            template_number: u32,
        ) -> Result<FirewallTemplate, Error> {
            Ok(self
                .go(api::get_firewall_template(template_number))
                .await?
                .0
                .into())
        }

        /// Create a new [`FirewallTemplate`].
        ///
        /// # Example
        /// ```rust,no_run
        /// # use std::net::Ipv4Addr;
        /// # use hrobot::models::{
        /// #     FirewallTemplateConfiguration, Rule, Rules, State, Ipv4Filter
        /// };
        /// # #[tokio::main]
        /// # async fn main() {
        /// let robot = hrobot::AsyncRobot::default();
        /// robot.create_firewall_template(FirewallTemplateConfiguration {
        ///     name: "My First Template".to_string(),
        ///     filter_ipv6: false,
        ///     whitelist_hetzner_services: true,
        ///     is_default: false,
        ///     rules: Rules {
        ///        ingress: vec![
        ///            Rule::accept("Allow from home").matching(
        ///                 Ipv4Filter::tcp(None)
        ///                     .from_ip(Ipv4Addr::new(123, 123, 123, 123))
        ///                     .to_port(27015..=27016)
        ///            )
        ///        ],
        ///        egress: vec![
        ///             Rule::accept("Allow all")
        ///        ]
        ///    },
        /// }).await.unwrap();
        /// # }
        /// ```
        pub async fn create_firewall_template(
            &self,
            template: FirewallTemplateConfiguration,
        ) -> Result<FirewallTemplate, Error> {
            Ok(self
                .go(api::create_firewall_template(template))
                .await?
                .0
                .into())
        }

        /// Delete a [`FirewallTemplate`].
        ///
        /// # Example
        /// ```rust,no_run
        /// # #[tokio::main]
        /// # async fn main() {
        /// let robot = hrobot::AsyncRobot::default();
        /// robot.delete_firewall_template(1234).await.unwrap();
        /// # }
        /// ```
        pub async fn delete_firewall_template(&self, template_number: u32) -> Result<(), Error> {
            self.go(api::delete_firewall_template(template_number))
                .await
                .or_else(|err| {
                    // Recover from error caused by attempting to deserialize ().
                    if matches!(err, Error::Deserialization(_)) {
                        Ok(())
                    } else {
                        Err(err)
                    }
                })
        }

        /// Modify a [`FirewallTemplate`].
        ///
        /// # Example
        /// ```rust,no_run
        /// # use hrobot::models::{FirewallTemplateConfiguration, Rules, Rule};
        /// # #[tokio::main]
        /// # async fn main() {
        /// let robot = hrobot::AsyncRobot::default();
        /// // Remove all firewall rules
        /// robot.update_firewall_template(1234, FirewallTemplateConfiguration {
        ///     name: "More like water-wall".to_string(),
        ///     filter_ipv6: false,
        ///     whitelist_hetzner_services: true,
        ///     is_default: false,
        ///     rules: Rules {
        ///        ingress: vec![],
        ///        egress: vec![]
        ///    },
        /// }).await.unwrap();
        /// # }
        /// ```
        pub async fn update_firewall_template(
            &self,
            template_number: u32,
            template: FirewallTemplateConfiguration,
        ) -> Result<FirewallTemplate, Error> {
            Ok(self
                .go(api::update_firewall_template(template_number, template))
                .await?
                .0
                .into())
        }
    }
}

#[cfg(feature = "async")]
pub use r#async::*;
