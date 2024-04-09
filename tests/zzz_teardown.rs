use std::time::Duration;

use hrobot::{
    api::server::{Cancel, Cancellation},
    AsyncRobot,
};
use serial_test::file_serial;
use tracing_test::traced_test;

mod common;

#[tokio::test]
#[traced_test]
#[file_serial]
async fn cancel_provisoned_server() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let server_id = common::provisioned_server_id().await;

    let Cancellation::Cancellable(cancellation) =
        robot.get_server_cancellation(server_id).await.unwrap()
    else {
        panic!("server has already been cancelled");
    };

    // Set cancellation for tomorrow.
    robot
        .cancel_server(
            server_id,
            Cancel {
                date: Some(cancellation.earliest_cancellation_date.next_day().unwrap()),
                reason: None,
                reserved: false,
            },
        )
        .await
        .unwrap();

    // Wait a while to make sure cancellation is in effect, and is also not immediate.
    tokio::time::sleep(Duration::from_secs(30)).await;

    robot.withdraw_server_cancellation(server_id).await.unwrap();

    tokio::time::sleep(Duration::from_secs(30)).await;

    // Cancel server immediately.
    robot
        .cancel_server(
            server_id,
            Cancel {
                date: None,
                reason: None,
                reserved: false,
            },
        )
        .await
        .unwrap();
}
