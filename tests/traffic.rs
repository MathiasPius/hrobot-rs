use std::net::IpAddr;

use hrobot::{api::traffic::TimeRange, AsyncRobot};
use ipnet::IpNet;
use time::Month;
use tracing::info;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn get_traffic_data() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let addresses: Vec<IpNet> = robot
        .list_servers()
        .await
        .unwrap()
        .into_iter()
        .map(|server| IpNet::from(IpAddr::from(server.ipv4.unwrap())))
        .collect();

    let traffic = robot
        .get_traffic(&addresses, TimeRange::month(2023, Month::July))
        .await
        .unwrap();

    info!("{traffic:#?}");
}
