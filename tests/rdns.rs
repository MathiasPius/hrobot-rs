mod common;

use hrobot::{
    error::{ApiError, Error},
    AsyncRobot,
};
use serial_test::file_serial;
use tracing::info;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
#[file_serial]
async fn list_rdns() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let rdns_entries = robot.list_rdns_entries().await.unwrap();
    info!("{rdns_entries:#?}");
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn get_rdns() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let rdns_entries = robot.list_rdns_entries().await.unwrap();
    info!("{rdns_entries:#?}");

    if let Some(entry) = rdns_entries.first() {
        let rdns = robot.get_rdns_entry(entry.ip).await.unwrap();

        info!("{rdns}");
    }
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn create_update_delete_rdns() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let subnets = robot.list_subnets().await.unwrap();
    info!("{subnets:#?}");

    let ip = subnets
        .into_iter()
        .filter(|(server_id, _)| *server_id == common::provisioned_server_id())
        .map(|(_, subnet)| subnet)
        .filter_map(|mut subnet| subnet.pop())
        .find_map(|subnet| {
            if subnet.ip.addr().is_ipv6() {
                Some(subnet.ip.addr())
            } else {
                None
            }
        });

    info!("{ip:#?}");

    if let Some(ip) = ip {
        assert!(matches!(
            robot.get_rdns_entry(ip).await,
            Err(Error::Api(ApiError::RdnsNotFound { .. }))
        ));

        let _ = robot
            .create_rdns_entry(ip, "test.example.com")
            .await
            .unwrap();

        assert_eq!(robot.get_rdns_entry(ip).await.unwrap(), "test.example.com");

        let _ = robot
            .update_rdns_entry(ip, "test2.example.com")
            .await
            .unwrap();

        assert_eq!(robot.get_rdns_entry(ip).await.unwrap(), "test2.example.com");

        robot.delete_rdns_entry(ip).await.unwrap();
    }
}
