mod common;

use hrobot::{
    api::boot::{Cpanel, CpanelConfig, CpanelDistribution},
    AsyncRobot,
};
use serial_test::file_serial;
use tracing::info;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
#[file_serial]
async fn get_cpanel_configuration() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let server = common::provisioned_server().await;
    // Fetch the complete server object, so we can get check
    // if the CPanel system is available for this server.
    let Some(availability) = robot.get_server(server.id).await.unwrap().availability else {
        return;
    };

    if availability.cpanel {
        let config = robot.get_cpanel_config(server.id).await.unwrap();
        info!("{config:#?}");
    }
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn last_cpanel_config() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let server = common::provisioned_server().await;

    let Some(availability) = server.availability else {
        return;
    };

    if availability.cpanel {
        let last_config = robot.get_last_cpanel_config(server.id).await.unwrap();
        info!("{last_config:#?}");
    }
}

#[tokio::test]
#[traced_test]
#[file_serial]
#[ignore = "enabling the Cpanel installation system is expensive, even if the system is never activated."]
async fn enable_disable_cpanel() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let server_id = common::provisioned_server_id().await;

    let mut activated_config = robot
        .enable_cpanel_config(
            server_id,
            CpanelConfig {
                distribution: CpanelDistribution::from("CentOS Stream"),
                language: "en_US".to_string(),
                hostname: "cpanel.example.com".to_string(),
            },
        )
        .await
        .unwrap();

    let config = robot.get_cpanel_config(server_id).await.unwrap();
    info!("{config:#?}");

    assert_eq!(Cpanel::Active(activated_config.clone()), config);

    let _ = robot.disable_cpanel_config(server_id).await.unwrap();

    assert!(matches!(
        robot.get_cpanel_config(server_id).await.unwrap(),
        Cpanel::Available(_)
    ));

    // We null out the password so we can compare to the latest
    // config, since the latest does not include passwords.
    activated_config.password = None;

    assert_eq!(
        robot.get_last_cpanel_config(server_id).await.unwrap(),
        activated_config
    );
}
