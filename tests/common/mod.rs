use hrobot::{
    api::{
        firewall::State,
        server::{Server, ServerId},
    },
    AsyncRobot,
};
use tracing::info;

#[allow(unused)]
pub fn provisioned_server_id() -> ServerId {
    dotenvy::dotenv().ok();

    ServerId(
        u32::from_str_radix(
            std::env::var("HETZNER_INTEGRATION_TEST_SERVER_ID")
                .as_deref()
                .unwrap(),
            10,
        )
        .unwrap(),
    )
}

#[allow(unused)]
pub async fn provisioned_server() -> Server {
    let id = provisioned_server_id();
    let robot = AsyncRobot::default();

    robot.get_server(id).await.unwrap()
}

#[allow(unused)]
pub async fn wait_firewall_ready(robot: &AsyncRobot, server_id: ServerId) {
    // Retry every 15 seconds, 10 times.
    let mut tries = 0;
    while tries < 20 {
        tries += 1;
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
        let firewall = robot.get_firewall(server_id).await.unwrap();
        if firewall.status != State::InProcess {
            break;
        } else {
            info!("Firewall state for {server_id} is still \"in process\", checking again in 15s.");
        }
    }
}
