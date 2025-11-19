use hrobot::{
    api::{
        boot::{Keyboard, Rescue, RescueConfig},
        reset::Reset,
        server::ServerId,
    },
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

async fn some_available_config(robot: &AsyncRobot, server_id: ServerId) -> RescueConfig {
    let Rescue::Available(mut available_configs) =
        robot.get_rescue_config(server_id).await.unwrap()
    else {
        panic!("no available rescue configurations");
    };
    println!("{available_configs:?}");

    RescueConfig {
        operating_system: available_configs.operating_systems.pop().unwrap(),
        authorized_keys: vec![],
        keyboard: Keyboard::US,
    }
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn enable_disable_vkvm() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let server_id = common::provisioned_server_id().await;

    let activated_config = robot
        .enable_rescue_config(server_id, some_available_config(&robot, server_id).await)
        .await
        .unwrap();

    let config = robot.get_rescue_config(server_id).await.unwrap();
    info!("{config:#?}");

    assert_eq!(Rescue::Active(activated_config.clone()), config);

    let _ = robot.disable_rescue_config(server_id).await.unwrap();

    assert!(matches!(
        robot.get_rescue_config(server_id).await.unwrap(),
        Rescue::Available(_)
    ));
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn test_vkvm_last() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let server_id = common::provisioned_server_id().await;

    let mut activated_config = robot
        .enable_rescue_config(server_id, some_available_config(&robot, server_id).await)
        .await
        .unwrap();

    let config = robot.get_rescue_config(server_id).await.unwrap();
    info!("{config:#?}");

    assert_eq!(Rescue::Active(activated_config.clone()), config);

    robot
        .trigger_reset(server_id, Reset::default())
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    assert_eq!(
        robot.get_last_rescue_config(server_id).await.unwrap(),
        activated_config
    );

    activated_config.password = None;
    tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    assert!(matches!(
        robot.get_rescue_config(server_id).await.unwrap(),
        Rescue::Available(_)
    ));
}
