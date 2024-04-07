mod common;

use hrobot::{
    api::boot::{Vnc, VncConfig, VncDistribution},
    AsyncRobot,
};
use serial_test::file_serial;
use tracing::info;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
#[file_serial]
async fn get_vnc_configuration() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let server_id = common::provisioned_server_id().await;

    let config = robot.get_vnc_config(server_id).await.unwrap();
    info!("{config:#?}");
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn test_enable_disable_vnc() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let server_id = common::provisioned_server_id().await;
    let mut activated_config = robot
        .enable_vnc_config(
            server_id,
            VncConfig {
                distribution: VncDistribution::from("CentOS-7.9"),
                language: "en_US".to_string(),
            },
        )
        .await
        .unwrap();

    let config = robot.get_vnc_config(server_id).await.unwrap();
    info!("{config:#?}");

    assert_eq!(Vnc::Active(activated_config.clone()), config);

    let _ = robot.disable_vnc_config(server_id).await.unwrap();

    assert!(matches!(
        robot.get_vnc_config(server_id).await.unwrap(),
        Vnc::Available(_)
    ));

    // We null out the password so we can compare to the latest
    // config, since the latest does not include passwords.
    activated_config.password = None;

    assert_eq!(
        robot.get_last_vnc_config(server_id).await.unwrap(),
        activated_config
    );
}
