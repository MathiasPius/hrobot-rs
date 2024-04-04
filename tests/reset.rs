use hrobot::{api::reset::Reset, AsyncRobot};
use tracing::info;
use tracing_test::traced_test;

mod common;

#[tokio::test]
#[traced_test]
async fn list_reset_options() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();
    let options = robot.list_reset_options().await.unwrap();

    info!("{options:#?}");
}

#[tokio::test]
#[traced_test]
async fn get_reset_options() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let reset_options = robot
        .get_reset_options(common::provisioned_server_id())
        .await
        .unwrap();

    info!("{reset_options:#?}");
}

#[tokio::test]
#[traced_test]
async fn trigger_reset() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    robot
        .trigger_reset(common::provisioned_server_id(), Reset::Hardware)
        .await
        .unwrap();
}
