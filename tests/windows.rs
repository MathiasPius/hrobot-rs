mod common;

use hrobot::{
    api::boot::{Windows, WindowsConfig, WindowsDistribution},
    AsyncRobot,
};
use serial_test::file_serial;
use tracing::info;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
#[file_serial]
async fn get_windows_configuration() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    // Fetch the complete server object, so we can get check
    // if the Windows system is available for this server.
    let server = common::provisioned_server().await;

    let Some(availability) = server.availability else {
        return;
    };

    if availability.windows {
        let config = robot.get_windows_config(server.id).await.unwrap();
        info!("{config:#?}");
    }
}

#[tokio::test]
#[traced_test]
#[file_serial]
#[ignore = "enabling the Windows installation system is expensive, even if the system is never activated."]
async fn enable_disable_windows() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let server_id = common::provisioned_server_id();

    let mut activated_config = robot
        .enable_windows_config(
            server_id,
            WindowsConfig {
                distribution: WindowsDistribution::from("standard"),
                language: "en_US".to_string(),
            },
        )
        .await
        .unwrap();

    let config = robot.get_windows_config(server_id).await.unwrap();
    info!("{config:#?}");

    assert_eq!(Windows::Active(activated_config.clone()), config);

    let _ = robot.disable_windows_config(server_id).await.unwrap();

    assert!(matches!(
        robot.get_windows_config(server_id).await.unwrap(),
        Windows::Available(_)
    ));

    // We null out the password so we can compare to the latest
    // config, since the latest does not include passwords.
    activated_config.password = None;

    assert_eq!(
        robot.get_last_windows_config(server_id).await.unwrap(),
        activated_config
    );
}
