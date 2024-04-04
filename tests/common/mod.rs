use std::time::Duration;

use hrobot::{
    api::{
        firewall::State,
        server::{Server, ServerId},
        storagebox::{StorageBox, StorageBoxId},
        vswitch::{ConnectionStatus, VSwitch, VSwitchId},
    },
    error::{ApiError, Error},
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
pub fn provisioned_storagebox_id() -> StorageBoxId {
    dotenvy::dotenv().ok();

    StorageBoxId(
        u32::from_str_radix(
            std::env::var("HETZNER_INTEGRATION_TEST_STORAGEBOX_ID")
                .as_deref()
                .unwrap(),
            10,
        )
        .unwrap(),
    )
}

#[allow(unused)]
pub async fn provisioned_storagebox() -> StorageBox {
    let robot = AsyncRobot::default();

    robot
        .get_storagebox(provisioned_storagebox_id())
        .await
        .unwrap()
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

#[allow(unused)]
pub async fn wait_vswitch_ready(robot: &AsyncRobot, id: VSwitchId) -> VSwitch {
    let mut tries = 20;
    loop {
        if tries == 0 {
            panic!("getting vswitch timed out");
        }

        match robot.get_vswitch(id).await {
            Ok(vswitch) => {
                // Ensure all servers are ready
                if vswitch
                    .servers
                    .iter()
                    .all(|server| server.status == ConnectionStatus::Ready)
                {
                    return vswitch;
                }
            }
            Err(Error::Api(ApiError::VswitchNotAvailable { .. })) => {
                info!("vswitch not available, waiting..");
            }
            Err(Error::Api(ApiError::VswitchInProcess { .. })) => {
                info!("vswitch in process, waiting..");
            }
            Err(err) => panic!("{}", err),
        };

        tokio::time::sleep(Duration::from_secs(15)).await;
        tries -= 1;
    }
}
