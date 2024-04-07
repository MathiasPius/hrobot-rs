mod common;

use std::time::Duration;

use bytesize::ByteSize;
use hrobot::{
    api::storagebox::{Accessibility, Permission, PlanStatus, SnapshotPlan},
    AsyncRobot,
};
use serial_test::file_serial;
use time::Weekday;
use tracing::info;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
#[file_serial]
async fn get_storageboxes() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let storageboxes = robot.list_storageboxes().await.unwrap();
    info!("{storageboxes:#?}");

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn get_storagebox() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let storageboxes = robot.list_storageboxes().await.unwrap();
    info!("{storageboxes:#?}");

    if let Some(storagebox) = storageboxes.last() {
        let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
        info!("{storagebox:#?}");
    }
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn list_snapshots() {
    let _ = dotenvy::dotenv().ok();

    common::provisioned_storagebox().await;

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn get_snapshotplans() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let storagebox = common::provisioned_storagebox().await;

    let plan = robot.get_snapshot_plan(storagebox.id).await.unwrap();
    info!("{plan:#?}");

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn list_subaccounts() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let storageboxes = robot.list_storageboxes().await.unwrap();
    info!("{storageboxes:#?}");

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    if let Some(storagebox) = storageboxes.first() {
        let accounts = robot.list_subaccounts(storagebox.id).await.unwrap();
        info!("{accounts:#?}");
    }

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn reset_password() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let storagebox = common::provisioned_storagebox().await;
    let password = robot
        .reset_storagebox_password(storagebox.id)
        .await
        .unwrap();
    info!("{password:#?}");

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn rename_storagebox() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let storagebox = common::provisioned_storagebox().await;
    robot
        .rename_storagebox(storagebox.id, "new-name")
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn toggle_all_settings() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let storagebox = common::provisioned_storagebox().await;

    // Don't act on storageboxes with data in them.
    if storagebox.disk.total != ByteSize::b(0) {
        panic!("storagebox has data in it, aborting");
    }

    let original_settings = storagebox.accessibility;

    // Test WebDAV
    if original_settings.webdav {
        let _ = robot
            .disable_storagebox_webdav(storagebox.id)
            .await
            .unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
        assert_ne!(storagebox.accessibility.webdav, original_settings.webdav);
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let _ = robot.enable_storagebox_webdav(storagebox.id).await.unwrap();
    } else {
        let _ = robot.enable_storagebox_webdav(storagebox.id).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
        assert_ne!(storagebox.accessibility.webdav, original_settings.webdav);
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let _ = robot
            .disable_storagebox_webdav(storagebox.id)
            .await
            .unwrap();
    }

    // Test Samba
    if original_settings.samba {
        let _ = robot.disable_storagebox_samba(storagebox.id).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
        assert_ne!(storagebox.accessibility.samba, original_settings.samba);
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let _ = robot.enable_storagebox_samba(storagebox.id).await.unwrap();
    } else {
        let _ = robot.enable_storagebox_samba(storagebox.id).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
        assert_ne!(storagebox.accessibility.samba, original_settings.samba);
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let _ = robot.disable_storagebox_samba(storagebox.id).await.unwrap();
    }

    // Test SSH
    if original_settings.ssh {
        let _ = robot.disable_storagebox_ssh(storagebox.id).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
        assert_ne!(storagebox.accessibility.ssh, original_settings.ssh);
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let _ = robot.enable_storagebox_ssh(storagebox.id).await.unwrap();
    } else {
        let _ = robot.enable_storagebox_ssh(storagebox.id).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
        assert_ne!(storagebox.accessibility.ssh, original_settings.ssh);
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let _ = robot.disable_storagebox_ssh(storagebox.id).await.unwrap();
    }

    // Test reachability
    if original_settings.external_reachability {
        let _ = robot
            .disable_storagebox_external_reachability(storagebox.id)
            .await
            .unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
        assert_ne!(
            storagebox.accessibility.external_reachability,
            original_settings.external_reachability
        );
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let _ = robot
            .enable_storagebox_external_reachability(storagebox.id)
            .await
            .unwrap();
    } else {
        let _ = robot
            .enable_storagebox_external_reachability(storagebox.id)
            .await
            .unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
        assert_ne!(
            storagebox.accessibility.external_reachability,
            original_settings.external_reachability
        );
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let _ = robot
            .disable_storagebox_external_reachability(storagebox.id)
            .await
            .unwrap();
    }
    tokio::time::sleep(std::time::Duration::from_secs(6)).await;

    // Test WebDAV
    if storagebox.snapshot_directory {
        let _ = robot
            .disable_storagebox_snapshot_directory(storagebox.id)
            .await
            .unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let _ = robot
            .enable_storagebox_snapshot_directory(storagebox.id)
            .await
            .unwrap();
    } else {
        let _ = robot
            .enable_storagebox_snapshot_directory(storagebox.id)
            .await
            .unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
        let _ = robot
            .disable_storagebox_snapshot_directory(storagebox.id)
            .await
            .unwrap();
    }
    tokio::time::sleep(std::time::Duration::from_secs(6)).await;

    // Reset all configurations.
    let _ = robot
        .configure_storagebox_accessibility(storagebox.id, original_settings.clone())
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(6)).await;

    let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
    assert_eq!(storagebox.accessibility, original_settings);

    return;
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn create_revert_delete_snapshot() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let storagebox = common::provisioned_storagebox().await;
    let snapshot = robot.create_snapshot(storagebox.id).await.unwrap();

    tokio::time::sleep(Duration::from_secs(6)).await;

    robot
        .revert_to_snapshot(storagebox.id, &snapshot.name)
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(6)).await;

    assert!(!robot
        .list_snapshots(storagebox.id)
        .await
        .unwrap()
        .is_empty());

    tokio::time::sleep(Duration::from_secs(6)).await;

    robot
        .delete_snapshot(storagebox.id, &snapshot.name)
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(6)).await;
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn create_comment_delete_snapshot() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let storagebox = common::provisioned_storagebox().await;
    let snapshot = robot.create_snapshot(storagebox.id).await.unwrap();

    tokio::time::sleep(Duration::from_secs(6)).await;

    robot
        .change_snapshot_comment(storagebox.id, &snapshot.name, "this is the updated comment")
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(6)).await;

    robot
        .delete_snapshot(storagebox.id, &snapshot.name)
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(6)).await;
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn update_snapshot_plans() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let storagebox = common::provisioned_storagebox().await;
    let plan = robot.get_snapshot_plan(storagebox.id).await.unwrap();

    if plan.status == PlanStatus::Disabled {
        // Daily
        let _ = robot
            .update_snapshot_plan(storagebox.id, SnapshotPlan::daily(10, 10))
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_secs(6)).await;

        let _ = robot
            .update_snapshot_plan(
                storagebox.id,
                SnapshotPlan::weekly(Weekday::Monday, 10, 10).with_limit(2),
            )
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_secs(6)).await;

        let plan = robot.get_snapshot_plan(storagebox.id).await.unwrap();

        assert_eq!(
            plan,
            SnapshotPlan::weekly(Weekday::Monday, 10, 10).with_limit(2)
        );

        tokio::time::sleep(Duration::from_secs(6)).await;

        let _ = robot
            .update_snapshot_plan(storagebox.id, SnapshotPlan::monthly(5, 10, 10))
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_secs(6)).await;

        let _ = robot
            .update_snapshot_plan(storagebox.id, SnapshotPlan::default())
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_secs(6)).await;

        return;
    }
}

#[tokio::test]
#[traced_test]
#[file_serial]
async fn create_update_delete_subaccount() {
    let _ = dotenvy::dotenv().ok();

    let robot = crate::AsyncRobot::default();

    let storagebox = common::provisioned_storagebox().await;

    let created_subaccount = robot
        .create_subaccount(
            storagebox.id,
            "/home/test-user",
            Accessibility::default(),
            Permission::ReadOnly,
            None,
        )
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(6)).await;

    let new_password = robot
        .reset_subaccount_password(storagebox.id, &created_subaccount.username)
        .await
        .unwrap();

    assert_ne!(new_password, created_subaccount.password);

    tokio::time::sleep(Duration::from_secs(6)).await;

    robot
        .update_subaccount(
            storagebox.id,
            &created_subaccount.username,
            "/home/dir",
            None,
            None,
            Some("test comment"),
        )
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(6)).await;

    robot
        .set_subaccount_home_directory(
            storagebox.id,
            &created_subaccount.username,
            "/homedirs/sub1",
        )
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(6)).await;

    robot
        .delete_subaccount(storagebox.id, created_subaccount.username)
        .await
        .unwrap();
}
