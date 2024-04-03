mod common;

use hrobot::AsyncRobot;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn rename_server() {
    let _ = dotenvy::dotenv().ok();
    let robot = AsyncRobot::default();

    let server = common::provisioned_server().await;

    let old_name = &server.name;
    let new_name = "test-rename";

    let renamed_server = robot.rename_server(server.id, new_name).await.unwrap();
    assert_eq!(renamed_server.name, new_name);

    let rolled_back_server = robot.rename_server(server.id, old_name).await.unwrap();
    assert_eq!(&rolled_back_server.name, old_name);
}
