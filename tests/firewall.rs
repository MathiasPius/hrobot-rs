use hrobot::{
    api::firewall::{Filter, FirewallTemplateConfig, Ipv4Filter, Rule, Rules},
    AsyncRobot,
};
use serial_test::file_serial;
use tracing::info;
use tracing_test::traced_test;

mod common;

#[tokio::test]
#[traced_test]
#[file_serial]
async fn get_firewall() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let server_id = common::provisioned_server_id();
    let firewall = robot.get_firewall(server_id).await.unwrap();

    info!("{firewall:#?}");
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn list_firewall_templates() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let templates = robot.list_firewall_templates().await.unwrap();
    info!("{templates:#?}");
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn get_firewall_template() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let templates = robot.list_firewall_templates().await.unwrap();
    info!("{templates:#?}");

    if let Some(template_ref) = templates.first() {
        let template = robot.get_firewall_template(template_ref.id).await.unwrap();
        info!("{template:#?}");
    }
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn set_firewall_configuration() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();
    let server = common::provisioned_server().await;
    common::wait_firewall_ready(&robot, server.id).await;

    // Fetch the current firewall configuration.
    let original_firewall = robot.get_firewall(server.id).await.unwrap();
    let mut config = original_firewall.config();

    // Apply a new rule
    common::wait_firewall_ready(&robot, server.id).await;
    let allow_1234 =
        Rule::accept("allow 1234").matching(Filter::Ipv4(Ipv4Filter::tcp(None).to_port(1234)));
    config.rules.ingress.push(allow_1234.clone());
    info!("{config:#?}");
    let _ = robot.set_firewall_config(server.id, &config).await.unwrap();

    info!("Waiting for firewall to be applied to {}", server.name);
    common::wait_firewall_ready(&robot, server.id).await;
    let applied_configuration = robot.get_firewall(server.id).await.unwrap();
    assert_eq!(
        applied_configuration.rules.ingress.last(),
        Some(&allow_1234)
    );

    // Revert to the original firewall config.
    let _ = robot
        .set_firewall_config(server.id, &original_firewall.config())
        .await
        .unwrap();
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn apply_firewall_template() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();
    let server = common::provisioned_server().await;

    // Fetch the current firewall configuration.
    let original_firewall = robot.get_firewall(server.id).await.unwrap();
    let config = original_firewall.config();
    info!("{config:#?}");

    // Create a firewall template that mimics the server's current configuration.
    // so as to be as non-disruptive as possible.
    let template = robot
        .create_firewall_template(config.to_template_config("test-template-from-config"))
        .await
        .unwrap();

    let _ = robot
        .apply_firewall_template(server.id, template.id)
        .await
        .unwrap();

    info!(
        "Waiting for firewall template to be applied to {}",
        server.name
    );

    common::wait_firewall_ready(&robot, server.id).await;
    let applied_configuration = robot.get_firewall(server.id).await.unwrap();
    assert_eq!(applied_configuration.rules, config.rules,);
    assert_eq!(applied_configuration.rules, template.rules);

    // Revert to the original firewall config.
    let _ = robot
        .set_firewall_config(server.id, &original_firewall.config())
        .await
        .unwrap();

    // Delete the temporary template.
    robot.delete_firewall_template(template.id).await.unwrap();
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn delete_firewall() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();
    let server = common::provisioned_server().await;

    common::wait_firewall_ready(&robot, server.id).await;

    // Fetch the current firewall configuration.
    let original_firewall = robot.get_firewall(server.id).await.unwrap();
    let original_config = original_firewall.config();
    info!("{original_config:#?}");

    let _ = robot.delete_firewall(server.id).await.unwrap();

    info!("Waiting for firewall to be applied to {}", server.name);

    common::wait_firewall_ready(&robot, server.id).await;
    let applied_configuration = robot.get_firewall(server.id).await.unwrap();
    // For some reason the default firewal has 2 identical "Allow all" rules.
    assert_eq!(applied_configuration.rules.ingress.len(), 2);
    assert_eq!(applied_configuration.rules.egress.len(), 1);

    // Revert to the original firewall config.
    let _ = robot
        .set_firewall_config(server.id, &original_firewall.config())
        .await
        .unwrap();
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn create_update_delete_firewall_template() {
    let _ = dotenvy::dotenv().ok();

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

    let _ = robot
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
