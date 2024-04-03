mod common;

use hrobot::{
    api::boot::{Linux, LinuxConfig, LinuxDistribution},
    AsyncRobot,
};
use serial_test::file_serial;
use tracing::info;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
#[file_serial]
async fn enable_disable_linux() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let server_id = common::provisioned_server_id();

    let mut activated_config = robot
        .enable_linux_config(
            server_id,
            LinuxConfig {
                distribution: LinuxDistribution::from("Arch Linux latest minimal"),
                language: "en".to_string(),
                authorized_keys: vec![],
            },
        )
        .await
        .unwrap();

    let config = robot.get_linux_config(server_id).await.unwrap();
    info!("{config:#?}");

    assert_eq!(Linux::Active(activated_config.clone()), config);

    let _ = robot.disable_linux_config(server_id).await.unwrap();

    assert!(matches!(
        robot.get_linux_config(server_id).await.unwrap(),
        Linux::Available(_)
    ));

    // We null out the password so we can compare to the latest
    // config, since the latest does not include passwords.
    activated_config.password = None;

    assert_eq!(
        robot.get_last_linux_config(server_id).await.unwrap(),
        activated_config
    );
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn get_linux_configuration() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let server_id = common::provisioned_server_id();

    let config = robot.get_linux_config(server_id).await.unwrap();
    info!("{config:#?}");
}
