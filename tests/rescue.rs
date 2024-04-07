use hrobot::{
    api::boot::{Keyboard, Rescue, RescueConfig, RescueOperatingSystem},
    AsyncRobot,
};
use serial_test::file_serial;
use tracing::info;
use tracing_test::traced_test;

mod common;

#[tokio::test]
#[traced_test]
#[file_serial]
async fn rescue_configuration() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let server_id = common::provisioned_server_id().await;
    let config = robot.get_rescue_config(server_id).await.unwrap();
    info!("{config:#?}");
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn enable_disable_vkvm() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let server = common::provisioned_server().await;

    let mut activated_config = robot
        .enable_rescue_config(
            server.id,
            RescueConfig {
                operating_system: RescueOperatingSystem::from("vkvm"),
                authorized_keys: vec![],
                keyboard: Keyboard::US,
            },
        )
        .await
        .unwrap();

    let config = robot.get_rescue_config(server.id).await.unwrap();
    info!("{config:#?}");

    assert_eq!(Rescue::Active(activated_config.clone()), config);

    let _ = robot.disable_rescue_config(server.id).await.unwrap();

    assert!(matches!(
        robot.get_rescue_config(server.id).await.unwrap(),
        Rescue::Available(_)
    ));

    // We null out the password so we can compare to the latest
    // config, since the latest does not include passwords.
    activated_config.password = None;

    assert_eq!(
        robot.get_last_rescue_config(server.id).await.unwrap(),
        activated_config
    );
}
