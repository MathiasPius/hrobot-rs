use hrobot::AsyncRobot;
use serial_test::file_serial;
use tracing::info;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
#[file_serial]
async fn test_create_delete_key() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let old_keys = robot.list_ssh_keys().await.unwrap();
    info!("{old_keys:#?}");

    // Create the new key
    let added_key = robot
        .create_ssh_key(
            "hrobot-rs-test-key",
            "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIEaQde8iCKizUOiXlowY1iEL1yCufgjb3aiatGQNPcHb",
        )
        .await
        .unwrap();

    // Fetch the (hopefully) updated key list
    let new_keys = robot.list_ssh_keys().await.unwrap();

    assert_eq!(new_keys.len(), old_keys.len() + 1);
    assert!(new_keys
        .into_iter()
        .find(|new_key| new_key.fingerprint == added_key.fingerprint)
        .is_some());

    // Rename the key
    let _ = robot
        .rename_ssh_key(&added_key.fingerprint, "new-key-name")
        .await
        .unwrap();

    // Get the key again, to check the name
    let fetched_key = robot.get_ssh_key(&added_key.fingerprint).await.unwrap();
    assert_eq!(fetched_key.fingerprint, added_key.fingerprint);

    assert_eq!(fetched_key.name, "new-key-name");

    // Clean up.
    robot.remove_ssh_key(&added_key.fingerprint).await.unwrap();
}
