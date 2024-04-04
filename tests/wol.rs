mod common;

use hrobot::AsyncRobot;
use tracing::info;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn wake_on_lan_available() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let server = common::provisioned_server().await;
    if robot.is_wake_on_lan_available(server.id).await.unwrap() {
        info!("{}: wake on lan is available", server.name);
    } else {
        info!("{}: wake on lan is NOT available", server.name);
    }
}
