//! Firewall & template structs and implementation.

mod models;
mod serde;

use crate::{error::Error, urlencode::UrlEncode, AsyncRobot};

use self::serde::*;
use ::serde::Serialize;
pub use models::*;

use super::{
    server::ServerId,
    wrapper::{Empty, List, Single},
    UnauthenticatedRequest,
};

pub(crate) fn get_firewall(
    server_number: ServerId,
) -> UnauthenticatedRequest<Single<InternalFirewall>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/{server_number}"
    ))
}

pub(crate) fn set_firewall_config(
    server_number: ServerId,
    firewall: &FirewallConfig,
) -> UnauthenticatedRequest<Single<InternalFirewall>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/{server_number}"
    ))
    .with_method("POST")
    .with_serialized_body(Into::<InternalFirewallConfig>::into(firewall).encode())
}

pub(crate) fn apply_firewall_template(
    server_number: ServerId,
    template_id: TemplateId,
) -> Result<UnauthenticatedRequest<Single<InternalFirewall>>, serde_html_form::ser::Error> {
    #[derive(Serialize)]
    struct ApplyTemplate {
        template_id: TemplateId,
    }

    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/{server_number}"
    ))
    .with_method("POST")
    .with_body(ApplyTemplate { template_id })
}

pub(crate) fn delete_firewall(
    server_number: ServerId,
) -> UnauthenticatedRequest<Single<InternalFirewall>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/{server_number}"
    ))
    .with_method("DELETE")
}

pub(crate) fn list_firewall_templates() -> UnauthenticatedRequest<List<FirewallTemplateReference>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/firewall/template")
}

pub(crate) fn get_firewall_template(
    template_number: TemplateId,
) -> UnauthenticatedRequest<Single<InternalFirewallTemplate>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/template/{template_number}"
    ))
}

pub(crate) fn create_firewall_template(
    template: FirewallTemplateConfig,
) -> UnauthenticatedRequest<Single<InternalFirewallTemplate>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/firewall/template")
        .with_method("POST")
        .with_serialized_body(Into::<InternalFirewallTemplateConfig>::into(template).encode())
}

pub(crate) fn delete_firewall_template(
    template_number: TemplateId,
) -> UnauthenticatedRequest<Empty> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/template/{template_number}"
    ))
    .with_method("DELETE")
}

pub(crate) fn update_firewall_template(
    template_number: TemplateId,
    template: FirewallTemplateConfig,
) -> UnauthenticatedRequest<Single<InternalFirewallTemplate>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/template/{template_number}"
    ))
    .with_method("POST")
    .with_serialized_body(Into::<InternalFirewallTemplateConfig>::into(template).encode())
}

impl AsyncRobot {
    /// Retrieve a [`Server`](crate::api::server::Server)'s [`Firewall`].
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// let firewall = robot.get_firewall(ServerId(1234567)).await.unwrap();
    /// println!("Ingress rule count: {}", firewall.rules.ingress.len());
    /// # }
    /// ```
    pub async fn get_firewall(&self, server_number: ServerId) -> Result<Firewall, Error> {
        Ok(self.go(get_firewall(server_number)).await?.0.into())
    }

    /// Replace a [`Server`](crate::api::server::Server)'s [`Firewall`] configuration.
    ///
    /// **Warning**: This replaces the entire firewall for
    /// both directions! If you don't define any ingress or
    /// egress rules, only the default-deny rule will apply!
    ///
    /// # Example
    /// ```rust,no_run
    /// # use std::net::Ipv4Addr;
    /// # use hrobot::api::server::ServerId;
    /// # use hrobot::api::firewall::{
    /// #     FirewallConfig, Rule, Rules, State, Ipv4Filter
    /// # };
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    ///
    /// let firewall = FirewallConfig {
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
    /// robot.set_firewall_config(ServerId(1234567), &firewall).await.unwrap();
    /// # }
    /// ```
    pub async fn set_firewall_config(
        &self,
        server_number: ServerId,
        firewall: &FirewallConfig,
    ) -> Result<Firewall, Error> {
        Ok(self
            .go(set_firewall_config(server_number, firewall))
            .await?
            .0
            .into())
    }

    /// Replace a [`Server`](crate::api::server::Server)'s [`Firewall`] configuration
    /// with the one defined in the given template.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use std::net::Ipv4Addr;
    /// # use hrobot::api::server::ServerId;
    /// # use hrobot::api::firewall::{
    /// #     FirewallConfig, Rule, Rules, State, Ipv4Filter,
    /// #     TemplateId,
    /// # };
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.apply_firewall_template(ServerId(1234567), TemplateId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn apply_firewall_template(
        &self,
        server_number: ServerId,
        template_id: TemplateId,
    ) -> Result<Firewall, Error> {
        Ok(self
            .go(apply_firewall_template(server_number, template_id)?)
            .await?
            .0
            .into())
    }

    /// Clear a [`Server`](crate::api::server::Server)s [`Firewall`] configuration.
    ///
    /// This reverts the server's firewall configuration to
    /// default Hetzner firewall, which has "Allow all" rules
    /// in both directions.
    ///  
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.delete_firewall(ServerId(1234567)).await.unwrap();
    /// # }
    /// ```
    pub async fn delete_firewall(&self, server_number: ServerId) -> Result<Firewall, Error> {
        Ok(self.go(delete_firewall(server_number)).await?.0.into())
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
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// let templates = robot.list_firewall_templates().await.unwrap();
    /// # }
    /// ```
    pub async fn list_firewall_templates(&self) -> Result<Vec<FirewallTemplateReference>, Error> {
        Ok(self.go(list_firewall_templates()).await?.0)
    }

    /// Retrieve a complete [`FirewallTemplate`].
    ///
    /// This returns the entire template, including its rules.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::firewall::TemplateId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// let template = robot.get_firewall_template(TemplateId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn get_firewall_template(
        &self,
        template_number: TemplateId,
    ) -> Result<FirewallTemplate, Error> {
        Ok(self
            .go(get_firewall_template(template_number))
            .await?
            .0
            .into())
    }

    /// Create a new [`FirewallTemplate`].
    ///
    /// # Example
    /// ```rust,no_run
    /// # use std::net::Ipv4Addr;
    /// # use hrobot::api::firewall::{
    /// #     FirewallTemplateConfig, Rule, Rules, State, Ipv4Filter
    /// };
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.create_firewall_template(FirewallTemplateConfig {
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
        template: FirewallTemplateConfig,
    ) -> Result<FirewallTemplate, Error> {
        Ok(self.go(create_firewall_template(template)).await?.0.into())
    }

    /// Delete a [`FirewallTemplate`].
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::firewall::TemplateId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.delete_firewall_template(TemplateId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn delete_firewall_template(&self, template_number: TemplateId) -> Result<(), Error> {
        self.go(delete_firewall_template(template_number))
            .await?
            .throw_away();
        Ok(())
    }

    /// Modify a [`FirewallTemplate`].
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::firewall::{FirewallTemplateConfig, Rules, Rule, TemplateId};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// // Remove all firewall rules
    /// robot.update_firewall_template(TemplateId(1234), FirewallTemplateConfig {
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
        template_number: TemplateId,
        template: FirewallTemplateConfig,
    ) -> Result<FirewallTemplate, Error> {
        Ok(self
            .go(update_firewall_template(template_number, template))
            .await?
            .0
            .into())
    }
}
