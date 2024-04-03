use hrobot::AsyncRobot;
use serial_test::file_serial;
use tracing::info;
use tracing_test::traced_test;

mod common;

#[tokio::test]
#[traced_test]
#[file_serial]
async fn boot_configuration() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let servers = robot.list_servers().await.unwrap();
    info!("{servers:#?}");

    if let Some(server) = servers.first() {
        let config = robot.get_boot_config(server.id).await.unwrap();
        info!("{config:#?}");
    }
}
