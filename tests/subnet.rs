use std::net::IpAddr;

use hrobot::AsyncRobot;
use tracing::info;
use tracing_test::traced_test;

mod common;

#[tokio::test]
#[traced_test]
async fn list_subnets() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();
    let subnets = robot.list_subnets().await.unwrap();

    info!("{subnets:#?}");
}

#[tokio::test]
#[traced_test]
async fn get_subnets() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();
    let subnets = robot.list_subnets().await.unwrap();
    info!("{subnets:#?}");

    let subnet = subnets
        .values()
        .into_iter()
        .find_map(|subnet| subnet.first());

    if let Some(subnet) = subnet {
        let subnet = robot.get_subnet(subnet.ip.addr()).await.unwrap();
        info!("{subnet:#?}");
    }
}

#[tokio::test]
#[traced_test]
async fn get_subnet_cancellation() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();
    let subnets = robot.list_subnets().await.unwrap();
    info!("{subnets:#?}");

    let subnet = subnets
        .values()
        .into_iter()
        .filter_map(|subnet| subnet.first())
        .find_map(|subnet| match subnet.ip.addr() {
            IpAddr::V4(addr) => Some(addr),
            _ => None,
        });

    if let Some(subnet) = subnet {
        let cancellation = robot.get_subnet_cancellation(subnet).await.unwrap();
        info!("{cancellation:#?}");
    }
}
