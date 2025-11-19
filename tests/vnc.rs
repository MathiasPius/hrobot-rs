mod common;

use core::panic;

use hrobot::{
    api::{
        boot::{Vnc, VncConfig},
        reset::Reset,
        server::ServerId,
    },
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

async fn some_available_config(robot: &AsyncRobot, server_id: ServerId) -> VncConfig {
    let Vnc::Available(mut available_configs) = robot.get_vnc_config(server_id).await.unwrap()
    else {
        panic!("no available vnc configurations");
    };
    println!("{available_configs:?}");

    VncConfig {
        distribution: available_configs.distributions.pop().unwrap(),
        language: available_configs.languages.pop().unwrap(),
    }
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn test_enable_disable_vnc() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let server_id = common::provisioned_server_id().await;

    let activated_config = robot
        .enable_vnc_config(server_id, some_available_config(&robot, server_id).await)
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
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn test_vnc_last() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let server_id = common::provisioned_server_id().await;
    let mut activated_config = robot
        .enable_vnc_config(server_id, some_available_config(&robot, server_id).await)
        .await
        .unwrap();

    let config = robot.get_vnc_config(server_id).await.unwrap();
    info!("{config:#?}");

    assert_eq!(Vnc::Active(activated_config.clone()), config);

    robot
        .trigger_reset(server_id, Reset::default())
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    assert_eq!(
        robot.get_last_vnc_config(server_id).await.unwrap(),
        activated_config
    );

    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    activated_config.password = None;

    assert!(matches!(
        robot.get_vnc_config(server_id).await.unwrap(),
        Vnc::Available(_)
    ));
}
