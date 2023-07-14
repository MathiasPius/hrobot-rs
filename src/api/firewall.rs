use crate::models::{Firewall, FirewallConfiguration, FirewallTemplate, FirewallTemplateReference};

use super::{
    wrapper::{List, Single},
    UnauthenticatedRequest,
};

pub(crate) fn get_firewall(server_number: u32) -> UnauthenticatedRequest<Single<Firewall>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/{server_number}"
    ))
}

pub(crate) fn set_firewall_configuration(
    server_number: u32,
    firewall: &FirewallConfiguration,
) -> Result<UnauthenticatedRequest<Single<Firewall>>, serde_html_form::ser::Error> {
    use crate::models::firewall::urlencode::UrlEncode;

    Ok(UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/{server_number}"
    ))
    .with_method("POST")
    .with_serialized_body(firewall.encode()))
}

pub(crate) fn delete_firewall(server_number: u32) -> UnauthenticatedRequest<Single<Firewall>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/{server_number}"
    ))
    .with_method("DELETE")
}

pub(crate) fn list_firewall_templates() -> UnauthenticatedRequest<List<FirewallTemplateReference>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/template"
    ))
}

pub(crate) fn get_firewall_template(
    template_number: u32,
) -> UnauthenticatedRequest<Single<FirewallTemplate>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/template/{template_number}"
    ))
}

#[cfg(all(test, feature = "hyper-client"))]
mod tests {
    use serial_test::serial;
    use tracing::info;
    use tracing_test::traced_test;

    use crate::models::{Action, Rule, State};

    #[tokio::test]
    #[traced_test]
    #[serial("firewall")]
    async fn test_get_firewall() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.iter().next() {
            let firewall = robot.get_firewall(server.id).await.unwrap();

            info!("{firewall:#?}");
        }
    }

    #[tokio::test]
    #[ignore = "unexpected failure might leave firewall in modified state."]
    #[traced_test]
    #[serial("firewall")]
    async fn test_set_firewall_configuration() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.iter().next() {
            // Fetch the current firewall configuration.
            let original_firewall = robot.get_firewall(server.id).await.unwrap();

            let mut config = original_firewall.configuration();

            // To not disturb the very real server, we'll just add an explicit discard
            // rule at the end, which theoretically should not interfere with the operation
            // of the server.
            let explicit_discard = Rule {
                name: "Explicit discard".to_owned(),
                action: Action::Discard,
                ..Default::default()
            };

            config.rules.ingress.push(explicit_discard.clone());

            info!("{config:#?}");

            robot
                .set_firewall_configuration(server.id, &config)
                .await
                .unwrap();

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
                .set_firewall_configuration(server.id, &original_firewall.configuration())
                .await
                .unwrap();
        }
    }

    #[tokio::test]
    #[ignore = "removing a production server's firewall, even temporarily, is obviously always *very* dangerous."]
    #[traced_test]
    #[serial("firewall")]
    async fn test_delete_firewall() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.iter().next() {
            // Fetch the current firewall configuration.
            let original_firewall = robot.get_firewall(server.id).await.unwrap();

            let original_config = original_firewall.configuration();

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
                .set_firewall_configuration(server.id, &original_firewall.configuration())
                .await
                .unwrap();
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

        if let Some(template_ref) = templates.iter().next() {
            let template = robot.get_firewall_template(template_ref.id).await.unwrap();
            info!("{template:#?}");
        }
    }
}
