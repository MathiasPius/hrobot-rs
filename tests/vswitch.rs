mod common;

use hrobot::{api::vswitch::VlanId, AsyncRobot};
use rand::{distributions::Alphanumeric, Rng as _};
use serial_test::file_serial;
use tracing::info;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
#[file_serial]
async fn list_vswitches() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let vswitches = robot.list_vswitches().await.unwrap();
    info!("{vswitches:#?}");
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn get_vswitch() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let vswitches = robot.list_vswitches().await.unwrap();
    info!("{vswitches:#?}");

    if let Some(vswitch) = vswitches.first() {
        let vswitch = robot.get_vswitch(vswitch.id).await.unwrap();
        info!("{vswitch:#?}");
    }
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn switch_end_to_end() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let unique_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let name = format!("hrobot-test-vswitch-{unique_id}");

    info!("using name {name}");

    // Find two available VLAN IDs we can use to create a new vSwitch
    // and then change the VLAN.
    let (first, second) = {
        let used_vlans: Vec<_> = robot
            .list_vswitches()
            .await
            .unwrap()
            .into_iter()
            .map(|switch| switch.vlan.0)
            .collect();

        let available_vlans: Vec<_> = (4030..4091)
            .filter(|vlan| !used_vlans.contains(vlan))
            .take(2)
            .collect();

        (available_vlans[0], available_vlans[1])
    };

    info!("using vlans {first} -> {second}");

    let vswitch = robot.create_vswitch(&name, VlanId(first)).await.unwrap();

    // Rename and change the VLAN ID.
    let name = format!("{name}-re");
    robot
        .update_vswitch(vswitch.id, &name, VlanId(second))
        .await
        .unwrap();

    let vswitch = common::wait_vswitch_ready(&robot, vswitch.id).await;

    assert_eq!(vswitch.name, name);
    assert_eq!(vswitch.vlan, VlanId(second));

    assert!(vswitch.subnets.is_empty());
    assert!(vswitch.servers.is_empty());
    assert!(vswitch.cloud_networks.is_empty());

    let server = common::provisioned_server().await;

    info!("connecting {server}", server = server.id);
    // Attempt to connect the server to the vswitch.
    robot
        .connect_vswitch_servers(vswitch.id, &[server.id])
        .await
        .unwrap();

    let connected_vswitch = common::wait_vswitch_ready(&robot, vswitch.id).await;

    assert_eq!(connected_vswitch.servers.len(), 1);
    assert_eq!(connected_vswitch.servers[0].id, server.id);

    // Disconnect the server again.
    robot
        .disconnect_vswitch_servers(vswitch.id, &[server.id])
        .await
        .unwrap();

    info!("disconnected again, waiting for vswitch availability");

    let disconnected_vswitch = common::wait_vswitch_ready(&robot, vswitch.id).await;

    assert!(disconnected_vswitch.servers.is_empty());

    robot.cancel_vswitch(vswitch.id, None).await.unwrap();
}
