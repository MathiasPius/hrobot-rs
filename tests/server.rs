mod common;

use hrobot::{
    api::server::{Cancellation, ServerId},
    error::{ApiError, Error},
    AsyncRobot,
};
use tracing::info;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn list_servers() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let servers = robot.list_servers().await.unwrap();
    info!("{servers:#?}");
}

#[tokio::test]
#[traced_test]
async fn get_server() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let server = common::provisioned_server().await;
    let retrieved_server = robot.get_server(server.id).await.unwrap();

    assert_eq!(retrieved_server.name, server.name);
}

#[tokio::test]
#[traced_test]
async fn get_nonexistent_server() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let result = robot.get_server(ServerId(1)).await;
    info!("{result:#?}");

    assert!(matches!(
        result,
        Err(Error::Api(ApiError::ServerNotFound { .. }))
    ));
}

#[tokio::test]
#[traced_test]
async fn get_server_cancellation() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let server = common::provisioned_server().await;
    let status = robot.get_server_cancellation(server.id).await.unwrap();
    info!("{status:#?}");
    assert!(matches!(status, Cancellation::Cancellable(_)));
}
