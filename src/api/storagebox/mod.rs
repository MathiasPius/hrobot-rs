use crate::{error::Error, AsyncRobot};

use super::{
    wrapper::{Empty, List, Single},
    UnauthenticatedRequest,
};

mod models;
pub use models::*;
use serde::Serialize;

fn list_storageboxes() -> UnauthenticatedRequest<List<StorageBoxReference>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/storagebox")
}

fn get_storagebox(storagebox: StorageBoxId) -> UnauthenticatedRequest<Single<StorageBox>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}"
    ))
}

fn reset_password(storagebox: StorageBoxId) -> UnauthenticatedRequest<Single<String>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}/password"
    ))
    .with_method("POST")
}

fn rename_storagebox(
    storagebox: StorageBoxId,
    name: &str,
) -> Result<UnauthenticatedRequest<Single<StorageBox>>, serde_html_form::ser::Error> {
    #[derive(Serialize)]
    struct RenameBox<'a> {
        storagebox_name: &'a str,
    }
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}"
    ))
    .with_method("POST")
    .with_body(RenameBox {
        storagebox_name: name,
    })
}

fn toggle_service(
    storagebox: StorageBoxId,
    service: &str,
    enabled: bool,
) -> UnauthenticatedRequest<Single<StorageBox>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}"
    ))
    .with_method("POST")
    .with_serialized_body(format!("{service}={enabled}"))
}

fn configure_services(
    storagebox: StorageBoxId,
    services: Services,
) -> Result<UnauthenticatedRequest<Single<StorageBox>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}"
    ))
    .with_method("POST")
    .with_body(services)
}

fn list_snapshots(storagebox: StorageBoxId) -> UnauthenticatedRequest<List<Snapshot>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}/snapshot"
    ))
}

fn create_snapshot(storagebox: StorageBoxId) -> UnauthenticatedRequest<Single<CreatedSnapshot>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}/snapshot"
    ))
    .with_method("POST")
}

fn delete_snapshot(storagebox: StorageBoxId, snapshot_name: &str) -> UnauthenticatedRequest<Empty> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}/snapshot/{snapshot_name}"
    ))
    .with_method("DELETE")
}

fn revert_to_snapshot(
    storagebox: StorageBoxId,
    snapshot_name: &str,
) -> UnauthenticatedRequest<Empty> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}/snapshot/{snapshot_name}"
    ))
    .with_method("POST")
    .with_serialized_body("revert=true".to_string())
}

fn change_snapshot_comment(
    storagebox: StorageBoxId,
    snapshot_name: &str,
    comment: &str,
) -> Result<UnauthenticatedRequest<Empty>, serde_html_form::ser::Error> {
    #[derive(Serialize)]
    struct ChangeComment<'a> {
        comment: &'a str,
    }
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}/snapshot/{snapshot_name}/comment"
    ))
    .with_method("POST")
    .with_body(ChangeComment { comment })
}

fn get_snapshot_plan(storagebox: StorageBoxId) -> UnauthenticatedRequest<Single<SnapshotPlan>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}/snapshotplan"
    ))
}

fn update_snapshot_plan(
    storagebox: StorageBoxId,
    plan: SnapshotPlan,
) -> Result<UnauthenticatedRequest<Single<SnapshotPlan>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}/snapshotplan"
    ))
    .with_method("POST")
    .with_body(plan)
}

impl AsyncRobot {
    /// List all storageboxes associated with this account.
    ///
    /// Note that this function returns a truncated version of the [`StorageBox`] which
    /// does not contain disk usage and service accessibility information.
    ///
    /// # Example
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.list_storageboxes().await.unwrap();
    /// # }
    /// ```
    pub async fn list_storageboxes(&self) -> Result<Vec<StorageBoxReference>, Error> {
        Ok(self.go(list_storageboxes()).await?.0)
    }

    /// Get a single storagebox.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_storagebox(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn get_storagebox(&self, id: StorageBoxId) -> Result<StorageBox, Error> {
        Ok(self.go(get_storagebox(id)).await?.0)
    }

    /// Rename storagebox.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.rename_storagebox(StorageBoxId(1234), "my-backups").await.unwrap();
    /// # }
    /// ```
    pub async fn rename_storagebox(
        &self,
        id: StorageBoxId,
        name: &str,
    ) -> Result<StorageBox, Error> {
        Ok(self.go(rename_storagebox(id, name)?).await?.0)
    }

    /// Configure storagebox accessibility.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::{StorageBoxId, Services};
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.configure_storagebox_services(StorageBoxId(1234), Services {
    ///     webdav: false,
    ///     samba: false,
    ///     ssh: false,
    ///     snapshot_directory: false,
    ///     external_reachability: false,
    /// }).await.unwrap();
    /// # }
    /// ```
    pub async fn configure_storagebox_services(
        &self,
        id: StorageBoxId,
        services: Services,
    ) -> Result<StorageBox, Error> {
        Ok(self.go(configure_services(id, services)?).await?.0)
    }

    /// Enable Samba (SMB) access to the storagebox.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_storagebox_samba(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_storagebox_samba(&self, id: StorageBoxId) -> Result<StorageBox, Error> {
        Ok(self.go(toggle_service(id, "samba", true)).await?.0)
    }

    /// Disable Samba (SMB) access to the storagebox.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_storagebox_samba(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_storagebox_samba(&self, id: StorageBoxId) -> Result<StorageBox, Error> {
        Ok(self.go(toggle_service(id, "samba", false)).await?.0)
    }

    /// Enable WebDAV access to the storagebox.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_storagebox_webdav(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_storagebox_webdav(&self, id: StorageBoxId) -> Result<StorageBox, Error> {
        Ok(self.go(toggle_service(id, "webdav", true)).await?.0)
    }

    /// Disable WebDAV access to the storagebox.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_storagebox_webdav(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_storagebox_webdav(&self, id: StorageBoxId) -> Result<StorageBox, Error> {
        Ok(self.go(toggle_service(id, "webdav", false)).await?.0)
    }

    /// Enable SSH access to the storagebox.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_storagebox_ssh(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_storagebox_ssh(&self, id: StorageBoxId) -> Result<StorageBox, Error> {
        Ok(self.go(toggle_service(id, "ssh", true)).await?.0)
    }

    /// Disable SSH access to the storagebox.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_storagebox_ssh(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_storagebox_ssh(&self, id: StorageBoxId) -> Result<StorageBox, Error> {
        Ok(self.go(toggle_service(id, "ssh", false)).await?.0)
    }

    /// Enable external reachability for the storagebox.
    ///
    /// External reachability means that the enabled services are reachable
    /// outside of Hetzner's networks. Without this enabled, you won't be able
    /// to log into the storagebox from anything other than Hetzner Cloud
    /// or Hetzner's Dedicated Servers.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_storagebox_external_reachability(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_storagebox_external_reachability(
        &self,
        id: StorageBoxId,
    ) -> Result<StorageBox, Error> {
        Ok(self
            .go(toggle_service(id, "external_reachability", true))
            .await?
            .0)
    }

    /// Disable external reachability for to the storagebox.
    ///
    /// External reachability means that the enabled services are reachable
    /// outside of Hetzner's networks. Without this enabled, you won't be able
    /// to log into the storagebox from anything other than Hetzner Cloud
    /// or Hetzner's Dedicated Servers.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_storagebox_external_reachability(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_storagebox_external_reachability(
        &self,
        id: StorageBoxId,
    ) -> Result<StorageBox, Error> {
        Ok(self
            .go(toggle_service(id, "external_reachability", false))
            .await?
            .0)
    }

    /// Enable snapshot directory visibility for the storagebox.
    ///
    /// When enabled, mounts a directory containing storage box snapshots
    /// in /.zfs or /home/.zfs depending on access method.
    ///
    /// Read more at: <https://docs.hetzner.com/robot/storage-box/snapshots/>
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_storagebox_snapshot_directory(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_storagebox_snapshot_directory(
        &self,
        id: StorageBoxId,
    ) -> Result<StorageBox, Error> {
        Ok(self.go(toggle_service(id, "zfs", true)).await?.0)
    }

    /// Disable snapshot directory visibility for the storagebox.
    ///
    /// When enabled, mounts a directory containing storage box snapshots
    /// in /.zfs or /home/.zfs depending on access method.
    ///
    /// Read more at: <https://docs.hetzner.com/robot/storage-box/snapshots/>
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_storagebox_snapshot_directory(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_storagebox_snapshot_directory(
        &self,
        id: StorageBoxId,
    ) -> Result<StorageBox, Error> {
        Ok(self.go(toggle_service(id, "zfs", false)).await?.0)
    }

    /// Reset storagebox password, returning the new password.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.reset_storagebox_password(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn reset_storagebox_password(&self, id: StorageBoxId) -> Result<String, Error> {
        Ok(self.go(reset_password(id)).await?.0)
    }

    /// List snapshots for storagebox.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.list_snapshots(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn list_snapshots(&self, id: StorageBoxId) -> Result<Vec<Snapshot>, Error> {
        Ok(self.go(list_snapshots(id)).await?.0)
    }

    /// Create a new snapshot of the storagebox.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.create_snapshot(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn create_snapshot(&self, id: StorageBoxId) -> Result<CreatedSnapshot, Error> {
        Ok(self.go(create_snapshot(id)).await?.0)
    }

    /// Delete a snapshot of the storagebox.
    ///
    /// Snapshots are named after the timestamp at which they are created
    /// with an implicit timezone of UTC. The safest way to target a snapshot
    /// for deletion, is to first retrieve it, and use its name from there.
    ///
    /// If you otherwise know the timestamp, but not the name of the snapshot,
    /// you can format it as "YYYY-MM-DDThh-mm-ss", with an assumed UTC timezone.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.delete_snapshot(
    ///     StorageBoxId(1234),
    ///     "2015-12-21T13-13-03"
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn delete_snapshot(
        &self,
        id: StorageBoxId,
        snapshot_name: &str,
    ) -> Result<(), Error> {
        self.go(delete_snapshot(id, snapshot_name)).await?;
        Ok(())
    }

    /// Revert storagebox to a snapshot.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.revert_to_snapshot(
    ///     StorageBoxId(1234),
    ///     "2015-12-21T13-13-03"
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn revert_to_snapshot(
        &self,
        id: StorageBoxId,
        snapshot_name: &str,
    ) -> Result<(), Error> {
        self.go(revert_to_snapshot(id, snapshot_name)).await?;
        Ok(())
    }

    /// Change snapshot comment.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.change_snapshot_comment(
    ///     StorageBoxId(1234),
    ///     "2015-12-21T13-13-03",
    ///     "Last backup before upgrade to 2.0"
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn change_snapshot_comment(
        &self,
        id: StorageBoxId,
        snapshot_name: &str,
        comment: &str,
    ) -> Result<(), Error> {
        self.go(change_snapshot_comment(id, snapshot_name, comment)?)
            .await?;
        Ok(())
    }

    /// Update snapshot plan for storagebox
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_snapshot_plan(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn get_snapshot_plan(&self, id: StorageBoxId) -> Result<SnapshotPlan, Error> {
        Ok(self.go(get_snapshot_plan(id)).await?.0)
    }

    /// Rename storagebox.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # use hrobot::time::Weekday;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.update_snapshot_plan(
    ///     StorageBoxId(1234),
    ///     SnapshotPlan::weekly(Weekday::Monday, 10, 0)
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn update_snapshot_plan(
        &self,
        id: StorageBoxId,
        plan: SnapshotPlan,
    ) -> Result<SnapshotPlan, Error> {
        Ok(self.go(update_snapshot_plan(id, plan)?).await?.0)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bytesize::ByteSize;
    use serial_test::serial;
    use time::{Month, Weekday};
    use tracing::info;
    use tracing_test::traced_test;

    use crate::api::storagebox::{PlanStatus, SnapshotPlan};

    #[tokio::test]
    #[traced_test]
    async fn test_get_storageboxes() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let storageboxes = robot.list_storageboxes().await.unwrap();
        info!("{storageboxes:#?}");
    }

    #[tokio::test]
    #[traced_test]
    #[serial("storagebox")]
    async fn test_get_storagebox() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let storageboxes = robot.list_storageboxes().await.unwrap();
        info!("{storageboxes:#?}");

        if let Some(storagebox) = storageboxes.last() {
            let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
            info!("{storagebox:#?}");
        }
    }

    #[tokio::test]
    #[traced_test]
    #[serial("storagebox")]
    #[ignore = "resets password, potentially breaking existing pasword-based clients"]
    async fn test_reset_password() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let storageboxes = robot.list_storageboxes().await.unwrap();
        info!("{storageboxes:#?}");

        if let Some(storagebox) = storageboxes.last() {
            let password = robot
                .reset_storagebox_password(storagebox.id)
                .await
                .unwrap();
            info!("{password:#?}");
        }
    }

    #[tokio::test]
    #[traced_test]
    #[ignore = "messes up enabled/disabled services for the storagebox, potentially leaving it in an unsafe state"]
    #[serial("storagebox")]
    async fn test_toggle_all_settings() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let storageboxes = robot.list_storageboxes().await.unwrap();
        info!("{storageboxes:#?}");

        for storagebox in storageboxes {
            let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();

            // Don't act on storageboxes with data in them.
            if storagebox.disk.total != ByteSize::b(0) {
                continue;
            }

            let original_settings = storagebox.services;

            // Test WebDAV
            if original_settings.webdav {
                robot
                    .disable_storagebox_webdav(storagebox.id)
                    .await
                    .unwrap();
            } else {
                robot.enable_storagebox_webdav(storagebox.id).await.unwrap();
            }
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
            assert_ne!(storagebox.services.webdav, original_settings.webdav);

            // Test Samba
            if original_settings.samba {
                robot.disable_storagebox_samba(storagebox.id).await.unwrap();
            } else {
                robot.enable_storagebox_samba(storagebox.id).await.unwrap();
            }
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
            assert_ne!(storagebox.services.samba, original_settings.samba);

            // Test SSH
            if original_settings.ssh {
                robot.disable_storagebox_ssh(storagebox.id).await.unwrap();
            } else {
                robot.enable_storagebox_ssh(storagebox.id).await.unwrap();
            }
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
            assert_ne!(storagebox.services.ssh, original_settings.ssh);

            // Test reachability
            if original_settings.external_reachability {
                robot
                    .disable_storagebox_external_reachability(storagebox.id)
                    .await
                    .unwrap();
            } else {
                robot
                    .enable_storagebox_external_reachability(storagebox.id)
                    .await
                    .unwrap();
            }
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
            assert_ne!(
                storagebox.services.external_reachability,
                original_settings.external_reachability
            );

            // Test snapshot directory
            if original_settings.snapshot_directory {
                robot
                    .disable_storagebox_snapshot_directory(storagebox.id)
                    .await
                    .unwrap();
            } else {
                robot
                    .enable_storagebox_snapshot_directory(storagebox.id)
                    .await
                    .unwrap();
            }
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
            assert_ne!(
                storagebox.services.snapshot_directory,
                original_settings.snapshot_directory
            );

            // Reset all configurations.
            robot
                .configure_storagebox_services(storagebox.id, original_settings.clone())
                .await
                .unwrap();

            tokio::time::sleep(std::time::Duration::from_secs(10)).await;

            let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
            assert_eq!(storagebox.services, original_settings);

            return;
        }
    }

    #[tokio::test]
    #[traced_test]
    #[serial("storagebox")]
    async fn test_list_snapshots() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let storageboxes = robot.list_storageboxes().await.unwrap();
        info!("{storageboxes:#?}");

        if let Some(storagebox) = storageboxes.last() {
            let snapshots = robot.list_snapshots(storagebox.id).await.unwrap();
            info!("{snapshots:#?}");
        }
    }

    #[tokio::test]
    #[traced_test]
    #[serial("storagebox")]
    #[ignore = "creating, reverting and deleting snapshots could lead to data loss"]
    async fn test_create_revert_delete_snapshot() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let storageboxes = robot.list_storageboxes().await.unwrap();
        info!("{storageboxes:#?}");

        if let Some(storagebox) = storageboxes.last() {
            let snapshot = robot.create_snapshot(storagebox.id).await.unwrap();

            tokio::time::sleep(Duration::from_secs(10)).await;

            robot
                .revert_to_snapshot(storagebox.id, &snapshot.name)
                .await
                .unwrap();

            tokio::time::sleep(Duration::from_secs(10)).await;

            robot
                .delete_snapshot(storagebox.id, &snapshot.name)
                .await
                .unwrap();

            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    }

    #[tokio::test]
    #[traced_test]
    #[serial("storagebox")]
    #[ignore = "creating and deleting snapshots could lead to data loss"]
    async fn test_create_comment_delete_snapshot() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let storageboxes = robot.list_storageboxes().await.unwrap();
        info!("{storageboxes:#?}");

        if let Some(storagebox) = storageboxes.last() {
            let snapshot = robot.create_snapshot(storagebox.id).await.unwrap();

            tokio::time::sleep(Duration::from_secs(10)).await;

            robot
                .change_snapshot_comment(
                    storagebox.id,
                    &snapshot.name,
                    "this is the updated comment",
                )
                .await
                .unwrap();

            tokio::time::sleep(Duration::from_secs(10)).await;

            robot
                .delete_snapshot(storagebox.id, &snapshot.name)
                .await
                .unwrap();

            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    }

    #[tokio::test]
    #[traced_test]
    async fn test_get_snapshotplans() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let storageboxes = robot.list_storageboxes().await.unwrap();
        info!("{storageboxes:#?}");

        for storagebox in storageboxes {
            let plan = robot.get_snapshot_plan(storagebox.id).await.unwrap();
            info!("{plan:#?}");
        }
    }

    #[tokio::test]
    #[traced_test]
    async fn test_set_snapshot_plans() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let storageboxes = robot.list_storageboxes().await.unwrap();
        info!("{storageboxes:#?}");

        for storagebox in storageboxes {
            let plan = robot.get_snapshot_plan(storagebox.id).await.unwrap();

            if plan.status == PlanStatus::Disabled {
                // Daily
                robot
                    .update_snapshot_plan(storagebox.id, SnapshotPlan::daily(10, 10))
                    .await
                    .unwrap();

                tokio::time::sleep(Duration::from_secs(10)).await;

                robot
                    .update_snapshot_plan(
                        storagebox.id,
                        SnapshotPlan::weekly(Weekday::Monday, 10, 10),
                    )
                    .await
                    .unwrap();

                tokio::time::sleep(Duration::from_secs(10)).await;

                robot
                    .update_snapshot_plan(storagebox.id, SnapshotPlan::monthly(5, 10, 10))
                    .await
                    .unwrap();

                tokio::time::sleep(Duration::from_secs(10)).await;

                robot
                    .update_snapshot_plan(storagebox.id, SnapshotPlan::default())
                    .await
                    .unwrap();

                tokio::time::sleep(Duration::from_secs(10)).await;

                return;
            }
        }
    }
}
