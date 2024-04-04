mod common;

use hrobot::{
    api::ip::TrafficWarnings,
    error::{ApiError, Error},
    AsyncRobot,
};
use serial_test::file_serial;
use tracing::info;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn list_ips() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();
    let ips = robot.list_ips().await.unwrap();

    info!("{ips:#?}");
}

#[tokio::test]
#[traced_test]
async fn get_server_ip_information() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let server = common::provisioned_server().await;
    let ip = robot.get_ip(server.ipv4.unwrap()).await.unwrap();
    info!("{ip:#?}");
}

#[tokio::test]
#[traced_test]
async fn get_separate_mac() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let server = common::provisioned_server().await;
    // Server primary IPs do not have configurable MAC addresses
    assert!(matches!(
        robot.get_ip_separate_mac(server.ipv4.unwrap()).await,
        Err(Error::Api(ApiError::MacNotAvailable { .. })),
    ));
}

#[tokio::test]
#[traced_test]
async fn get_server_ip_cancellation() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let ip = common::provisioned_server().await.ipv4.unwrap();
    let cancellation = robot.get_ip_cancellation(ip).await.unwrap();
    info!("{cancellation:#?}");
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn enable_and_disable_traffic_warnings() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let server = common::provisioned_server().await;
    let ip = server.ipv4.unwrap();
    let ip = robot.get_ip(ip).await.unwrap();
    info!("{ip:#?}");

    let original_traffic_warning = ip.traffic_warnings;

    let new_warnings = robot
        .enable_ip_traffic_warnings(ip.ip, Some(TrafficWarnings::default()))
        .await
        .unwrap();

    assert_eq!(
        new_warnings.traffic_warnings.unwrap(),
        TrafficWarnings::default()
    );

    let _ = robot.disable_ip_traffic_warnings(ip.ip).await.unwrap();

    // Restore the original traffic warning settings.
    if let Some(warnings) = original_traffic_warning {
        let _ = robot
            .enable_ip_traffic_warnings(ip.ip, Some(warnings))
            .await
            .unwrap();
    }
}
