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
    /// # dotenvy::dotenv().ok();
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
        self.go(delete_firewall_template(template_number)).await?;
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

#[cfg(test)]
mod tests {

    #[cfg(feature = "non-disruptive-tests")]
    mod non_disruptive_tests {

        use serial_test::serial;
        use tracing::info;
        use tracing_test::traced_test;

        #[tokio::test]
        #[traced_test]
        #[serial("firewall")]
        async fn test_get_firewall() {
            dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let servers = robot.list_servers().await.unwrap();
            info!("{servers:#?}");

            if let Some(server) = servers.first() {
                let firewall = robot.get_firewall(server.id).await.unwrap();

                info!("{firewall:#?}");
            }
        }

        #[tokio::test]
        #[traced_test]
        #[serial("firewall-templates")]
        async fn test_list_firewall_templates() {
            dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let templates = robot.list_firewall_templates().await.unwrap();
            info!("{templates:#?}");
        }

        #[tokio::test]
        #[traced_test]
        #[serial("firewall-templates")]
        async fn test_get_firewall_template() {
            dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let templates = robot.list_firewall_templates().await.unwrap();
            info!("{templates:#?}");

            if let Some(template_ref) = templates.first() {
                let template = robot.get_firewall_template(template_ref.id).await.unwrap();
                info!("{template:#?}");
            }
        }
    }

    #[cfg(feature = "disruptive-tests")]
    mod disruptive_tests {
        use serial_test::serial;
        use tracing::info;
        use tracing_test::traced_test;

        use crate::api::firewall::{FirewallTemplateConfig, Rule, Rules, State};

        #[tokio::test]
        #[traced_test]
        #[serial("firewall")]
        #[ignore = "unexpected failure might leave firewall in modified state."]
        async fn test_set_firewall_configuration() {
            dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let servers = robot.list_servers().await.unwrap();
            info!("{servers:#?}");

            if let Some(server) = servers.first() {
                // Fetch the current firewall configuration.
                let original_firewall = robot.get_firewall(server.id).await.unwrap();

                let mut config = original_firewall.config();

                // To not disturb the very real server, we'll just add an explicit discard
                // rule at the end, which theoretically should not interfere with the operation
                // of the server.
                let explicit_discard = Rule::discard("Explicit discard");

                config.rules.ingress.push(explicit_discard.clone());

                info!("{config:#?}");

                robot.set_firewall_config(server.id, &config).await.unwrap();

                info!("Waiting for firewall to be applied to {}", server.name);

                // Retry every 30 seconds, 10 times.
                let mut tries = 0;
                while tries < 10 {
                    tries += 1;
                    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                    let firewall = robot.get_firewall(server.id).await.unwrap();
                    if firewall.status != State::InProcess {
                        break;
                    } else {
                        info!("Firewall state is still \"in process\", checking again in 30s.");
                    }
                }

                let applied_configuration = robot.get_firewall(server.id).await.unwrap();

                assert_eq!(
                    applied_configuration.rules.ingress.last(),
                    Some(&explicit_discard)
                );

                // Revert to the original firewall config.
                robot
                    .set_firewall_config(server.id, &original_firewall.config())
                    .await
                    .unwrap();
            }
        }

        #[tokio::test]
        #[traced_test]
        #[serial("firewall")]
        #[ignore = "unexpected failure might leave firewall in modified state."]
        async fn test_apply_firewall_template() {
            dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let servers = robot.list_servers().await.unwrap();
            info!("{servers:#?}");

            if let Some(server) = servers.first() {
                // Fetch the current firewall configuration.
                let original_firewall = robot.get_firewall(server.id).await.unwrap();

                let config = original_firewall.config();

                info!("{config:#?}");

                // Create a firewall template that mimics the server's current configuration.
                // so as to be as non-disruptive as possible.
                let template = robot
                    .create_firewall_template(
                        config.to_template_config("test-template-from-config"),
                    )
                    .await
                    .unwrap();

                robot
                    .apply_firewall_template(server.id, template.id)
                    .await
                    .unwrap();

                info!(
                    "Waiting for firewall template to be applied to {}",
                    server.name
                );

                // Retry every 30 seconds, 10 times.
                let mut tries = 0;
                while tries < 10 {
                    tries += 1;
                    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                    let firewall = robot.get_firewall(server.id).await.unwrap();
                    if firewall.status != State::InProcess {
                        break;
                    } else {
                        info!("Firewall state is still \"in process\", checking again in 30s.");
                    }
                }

                let applied_configuration = robot.get_firewall(server.id).await.unwrap();

                assert_eq!(applied_configuration.rules, config.rules,);

                assert_eq!(applied_configuration.rules, template.rules);

                // Revert to the original firewall config.
                robot
                    .set_firewall_config(server.id, &original_firewall.config())
                    .await
                    .unwrap();

                // Delete the temporary template.
                robot.delete_firewall_template(template.id).await.unwrap();
            }
        }

        #[tokio::test]
        #[traced_test]
        #[serial("firewall")]
        #[ignore = "removing a production server's firewall, even temporarily, is obviously always *very* dangerous."]
        async fn test_delete_firewall() {
            dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let servers = robot.list_servers().await.unwrap();
            info!("{servers:#?}");

            if let Some(server) = servers.first() {
                // Fetch the current firewall configuration.
                let original_firewall = robot.get_firewall(server.id).await.unwrap();

                let original_config = original_firewall.config();

                info!("{original_config:#?}");

                robot.delete_firewall(server.id).await.unwrap();

                info!("Waiting for firewall to be applied to {}", server.name);

                // Retry every 30 seconds, 10 times.
                let mut tries = 0;
                while tries < 10 {
                    tries += 1;
                    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                    let firewall = robot.get_firewall(server.id).await.unwrap();
                    if firewall.status != State::InProcess {
                        break;
                    } else {
                        info!("Firewall state is still \"in process\", checking again in 30s.");
                    }
                }

                let applied_configuration = robot.get_firewall(server.id).await.unwrap();

                // For some reason the default firewal has 2 identical "Allow all" rules.
                assert_eq!(applied_configuration.rules.ingress.len(), 2);
                assert_eq!(applied_configuration.rules.egress.len(), 1);

                // Revert to the original firewall config.
                robot
                    .set_firewall_config(server.id, &original_firewall.config())
                    .await
                    .unwrap();
            }
        }

        #[tokio::test]
        #[ignore = "unexpected failure could leave template behind."]
        #[traced_test]
        #[serial("firewall-templates")]
        async fn test_create_update_delete_firewall_template() {
            dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let template = robot
                .create_firewall_template(FirewallTemplateConfig {
                    name: "Lockdown".to_string(),
                    filter_ipv6: false,
                    whitelist_hetzner_services: false,
                    is_default: false,
                    rules: Rules {
                        ingress: vec![Rule::discard("Deny in")],
                        egress: vec![Rule::discard("Deny out")],
                    },
                })
                .await
                .unwrap();

            robot
                .update_firewall_template(
                    template.id,
                    FirewallTemplateConfig {
                        name: "Come on in".to_string(),
                        filter_ipv6: false,
                        whitelist_hetzner_services: true,
                        is_default: false,
                        rules: Rules {
                            ingress: vec![Rule::accept("Allow in")],
                            egress: vec![Rule::accept("Allow out")],
                        },
                    },
                )
                .await
                .unwrap();

            robot.delete_firewall_template(template.id).await.unwrap();
        }
    }
}
