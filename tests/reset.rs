use hrobot::AsyncRobot;
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

    let server = common::provisioned_server().await;
    let reset_options = robot.get_reset_options(server.id).await.unwrap();

    info!("{reset_options:#?}");
}
