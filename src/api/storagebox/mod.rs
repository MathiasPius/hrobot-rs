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

fn configure_accessibility(
    storagebox: StorageBoxId,
    accessibility: Accessibility,
) -> Result<UnauthenticatedRequest<Single<StorageBox>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}"
    ))
    .with_method("POST")
    .with_body(accessibility)
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

fn list_subaccounts(storagebox: StorageBoxId) -> UnauthenticatedRequest<List<Subaccount>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}/subaccount"
    ))
}

#[derive(Serialize)]
struct SubaccountConfig<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    homedirectory: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    samba: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ssh: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    webdav: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    readonly: Option<Permission>,

    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<&'a str>,
}

fn create_subaccount(
    storagebox: StorageBoxId,
    home_directory: &str,
    accessibility: Accessibility,
    read_only: Permission,
    comment: Option<&str>,
) -> Result<UnauthenticatedRequest<Single<CreatedSubaccount>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}/subaccount"
    ))
    .with_method("POST")
    .with_body(SubaccountConfig {
        homedirectory: Some(home_directory),
        samba: Some(accessibility.samba),
        ssh: Some(accessibility.ssh),
        webdav: Some(accessibility.webdav),
        readonly: Some(read_only),
        comment,
    })
}

fn update_subaccount(
    storagebox: StorageBoxId,
    subaccount: SubaccountId,
    home_directory: Option<&str>,
    accessibility: Option<&Accessibility>,
    read_only: Option<Permission>,
    comment: Option<&str>,
) -> Result<UnauthenticatedRequest<Empty>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}/subaccount/{subaccount}"
    ))
    .with_method("PUT")
    .with_body(SubaccountConfig {
        homedirectory: home_directory,
        samba: accessibility.map(|a| a.samba),
        ssh: accessibility.map(|a| a.ssh),
        webdav: accessibility.map(|a| a.webdav),
        readonly: read_only,
        comment,
    })
}

fn delete_subaccount(
    storagebox: StorageBoxId,
    subaccount: SubaccountId,
) -> UnauthenticatedRequest<Empty> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}/subaccount/{subaccount}"
    ))
    .with_method("DELETE")
}

fn reset_subaccount_password(
    storagebox: StorageBoxId,
    subaccount: SubaccountId,
) -> UnauthenticatedRequest<Single<String>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}/subaccount/{subaccount}/password"
    ))
    .with_method("POST")
}

impl AsyncRobot {
    /// List all storageboxes associated with this account.
    ///
    /// Note that this function returns a truncated version of the [`StorageBox`] which
    /// does not contain disk usage and service accessibility information.
    ///
    /// # Example
    /// ```rust,no_run
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
    /// # use hrobot::api::storagebox::{StorageBoxId, Accessibility};
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.configure_storagebox_accessibility(StorageBoxId(1234), Accessibility {
    ///     webdav: false,
    ///     samba: false,
    ///     ssh: false,
    ///     external_reachability: false,
    /// }).await.unwrap();
    /// # }
    /// ```
    pub async fn configure_storagebox_accessibility(
        &self,
        id: StorageBoxId,
        accessibility: Accessibility,
    ) -> Result<StorageBox, Error> {
        Ok(self
            .go(configure_accessibility(id, accessibility)?)
            .await?
            .0)
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

    /// Update snapshot plan.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::{StorageBoxId, SnapshotPlan};
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

    /// List sub-accounts for storagebox.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::StorageBoxId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.list_subaccounts(StorageBoxId(1234)).await.unwrap();
    /// # }
    /// ```
    pub async fn list_subaccounts(&self, id: StorageBoxId) -> Result<Vec<Subaccount>, Error> {
        Ok(self.go(list_subaccounts(id)).await?.0)
    }

    /// Delete sub-account.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::{StorageBoxId, SubaccountId};
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.delete_subaccount(
    ///     StorageBoxId(1234),
    ///     SubaccountId("u1234-sub1".to_string()),
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn create_subaccount(
        &self,
        storagebox: StorageBoxId,
        home_directory: &str,
        accessibility: Accessibility,
        permissions: Permission,
        comment: Option<&str>,
    ) -> Result<CreatedSubaccount, Error> {
        Ok(self
            .go(create_subaccount(
                storagebox,
                home_directory,
                accessibility,
                permissions,
                comment,
            )?)
            .await?
            .0)
    }

    /// Configure storagebox sub-account accessibility.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::{StorageBoxId, SubaccountId, Accessibility};
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.configure_subaccount_accessibility(
    ///     StorageBoxId(1234),
    ///     SubaccountId("u1234-sub1".to_string()),
    ///     Accessibility {
    ///         webdav: false,
    ///         samba: false,
    ///         ssh: false,
    ///         external_reachability: false,
    ///     }
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn configure_subaccount_accessibility(
        &self,
        storagebox: StorageBoxId,
        subaccount: SubaccountId,
        accessibility: Accessibility,
    ) -> Result<(), Error> {
        self.go(update_subaccount(
            storagebox,
            subaccount,
            None,
            Some(&accessibility),
            None,
            None,
        )?)
        .await?;
        Ok(())
    }

    /// Change home directory of storagebox sub-account
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::{StorageBoxId, SubaccountId, Accessibility};
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.set_subaccount_home_directory(
    ///     StorageBoxId(1234),
    ///     SubaccountId("u1234-sub1".to_string()),
    ///     "/homedirs/sub1"
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn set_subaccount_home_directory(
        &self,
        storagebox: StorageBoxId,
        subaccount: SubaccountId,
        home_directory: &str,
    ) -> Result<(), Error> {
        self.go(update_subaccount(
            storagebox,
            subaccount,
            Some(home_directory),
            None,
            None,
            None,
        )?)
        .await?;
        Ok(())
    }

    /// Change write permission of sub-account.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::{StorageBoxId, SubaccountId, Accessibility, Permission};
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.set_subaccount_permissions(
    ///     StorageBoxId(1234),
    ///     SubaccountId("u1234-sub1".to_string()),
    ///     Permission::ReadOnly,
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn set_subaccount_permissions(
        &self,
        storagebox: StorageBoxId,
        subaccount: SubaccountId,
        permissions: Permission,
    ) -> Result<(), Error> {
        self.go(update_subaccount(
            storagebox,
            subaccount,
            None,
            None,
            Some(permissions),
            None,
        )?)
        .await?;
        Ok(())
    }

    /// Change sub-account comment/description.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::{StorageBoxId, SubaccountId};
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.set_subaccount_comment(
    ///     StorageBoxId(1234),
    ///     SubaccountId("u1234-sub1".to_string()),
    ///     "Sub-account used for accessing backups"
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn set_subaccount_comment(
        &self,
        storagebox: StorageBoxId,
        subaccount: SubaccountId,
        comment: &str,
    ) -> Result<(), Error> {
        self.go(update_subaccount(
            storagebox,
            subaccount,
            None,
            None,
            None,
            Some(comment),
        )?)
        .await?;
        Ok(())
    }

    /// Reset sub-account password.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::{StorageBoxId, SubaccountId};
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// let password = robot.reset_subaccount_password(
    ///     StorageBoxId(1234),
    ///     SubaccountId("u1234-sub1".to_string()),
    /// ).await.unwrap();
    ///
    /// println!("new password: {password}");
    /// # }
    /// ```
    pub async fn reset_subaccount_password(
        &self,
        storagebox: StorageBoxId,
        subaccount: SubaccountId,
    ) -> Result<String, Error> {
        Ok(self
            .go(reset_subaccount_password(storagebox, subaccount))
            .await?
            .0)
    }

    /// Delete sub-account.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::storagebox::{StorageBoxId, SubaccountId};
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.delete_subaccount(
    ///     StorageBoxId(1234),
    ///     SubaccountId("u1234-sub1".to_string()),
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn delete_subaccount(
        &self,
        storagebox: StorageBoxId,
        subaccount: SubaccountId,
    ) -> Result<(), Error> {
        self.go(delete_subaccount(storagebox, subaccount)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "non-disruptive-tests")]
    mod non_disruptive_tests {
        use serial_test::serial;
        use tracing::info;
        use tracing_test::traced_test;

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
        async fn test_list_subaccounts() {
            dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let storageboxes = robot.list_storageboxes().await.unwrap();
            info!("{storageboxes:#?}");

            if let Some(storagebox) = storageboxes.first() {
                let accounts = robot.list_subaccounts(storagebox.id).await.unwrap();
                info!("{accounts:#?}");
            }
        }
    }

    #[cfg(feature = "disruptive-tests")]
    mod disruptive_tests {
        use std::time::Duration;

        use bytesize::ByteSize;
        use serial_test::serial;
        use time::Weekday;
        use tracing::info;
        use tracing_test::traced_test;

        use crate::api::storagebox::{PlanStatus, SnapshotPlan};

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
        #[serial("storagebox")]
        #[ignore = "messes up enabled/disabled services for the storagebox, potentially leaving it in an unsafe state"]
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

                let original_settings = storagebox.accessibility;

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
                assert_ne!(storagebox.accessibility.webdav, original_settings.webdav);

                // Test Samba
                if original_settings.samba {
                    robot.disable_storagebox_samba(storagebox.id).await.unwrap();
                } else {
                    robot.enable_storagebox_samba(storagebox.id).await.unwrap();
                }
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
                assert_ne!(storagebox.accessibility.samba, original_settings.samba);

                // Test SSH
                if original_settings.ssh {
                    robot.disable_storagebox_ssh(storagebox.id).await.unwrap();
                } else {
                    robot.enable_storagebox_ssh(storagebox.id).await.unwrap();
                }
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
                assert_ne!(storagebox.accessibility.ssh, original_settings.ssh);

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
                    storagebox.accessibility.external_reachability,
                    original_settings.external_reachability
                );

                // Reset all configurations.
                robot
                    .configure_storagebox_accessibility(storagebox.id, original_settings.clone())
                    .await
                    .unwrap();

                tokio::time::sleep(std::time::Duration::from_secs(10)).await;

                let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
                assert_eq!(storagebox.accessibility, original_settings);

                return;
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
        #[serial("storagebox")]
        #[ignore = "replaces the snapshot plan of the storage box."]
        async fn test_update_snapshot_plans() {
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
}
