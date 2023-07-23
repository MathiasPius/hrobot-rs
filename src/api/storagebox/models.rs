use std::fmt::Display;

use bytesize::ByteSize;
use serde::{Deserialize, Serialize};
use time::{Date, Month, OffsetDateTime, Weekday};

use crate::api::server::ServerId;

/// Unique StorageBox ID.
///
/// Simple wrapper around a u32, to avoid confusion with for example [`ServerId`](crate::api::server::ServerId)
/// and to make it intuitive what kind of argument you need to give to functions like [`AsyncRobot::get_storagebox`](crate::AsyncRobot::get_storagebox()).
///
/// Using a plain integer means it isn't clear what the argument is, is it a counter of my servers, where the argument
/// is in range `0..N` where `N` is the number of storageboxes in my account, or is it a limiter, like get first `N`
/// storageboxes, for example.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StorageBoxId(pub u32);

impl From<u32> for StorageBoxId {
    fn from(value: u32) -> Self {
        StorageBoxId(value)
    }
}

impl From<StorageBoxId> for u32 {
    fn from(value: StorageBoxId) -> Self {
        value.0
    }
}

impl Display for StorageBoxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<u32> for StorageBoxId {
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

/// Reference to a storagebox.
///
/// Does not contain disk, access or reachability information.
#[derive(Debug, Clone, Deserialize)]
pub struct StorageBoxReference {
    /// Unique ID for this storagebox.
    pub id: StorageBoxId,

    /// Login/username used for accessing the storagebox.
    pub login: String,

    /// Human-readable name for the storage box.
    pub name: String,

    /// Product name, such as `BX06`.
    pub product: String,

    /// Indicates whether the storagebox has been cancelled.
    pub cancelled: bool,

    /// Indicates if the storagebox is locked.
    pub locked: bool,

    /// Datacenter location of the storagebox, e.g. `FSN1`.
    pub location: String,

    /// Server which this storagebox is linked to.
    pub linked_server: Option<ServerId>,

    /// Date until which this storagebox has been paid for.
    pub paid_until: Date,
}

/// Storage Box
#[derive(Debug, Clone, Deserialize)]
pub struct StorageBox {
    /// Unique ID for this storagebox.
    pub id: StorageBoxId,

    /// Login/username used for accessing the storagebox.
    pub login: String,

    /// Human-readable name for the storage box.
    pub name: String,

    /// Product name, such as `BX06`.
    pub product: String,

    /// Indicates whether the storagebox has been cancelled.
    pub cancelled: bool,

    /// Indicates if the storagebox is locked.
    pub locked: bool,

    /// Datacenter location of the storagebox, e.g. `FSN1`.
    pub location: String,

    /// Server which this storagebox is linked to.
    pub linked_server: Option<ServerId>,

    /// Date until which this storagebox has been paid for.
    pub paid_until: Date,

    /// Servername through which the storagebox can be accessed.
    pub server: String,

    /// Name of the host system for the storagebox.
    pub host_system: String,

    /// Disk usage and quota.
    #[serde(flatten)]
    pub disk: Disk,

    /// Accessibility.
    #[serde(flatten)]
    pub services: Services,
}

/// Disk usage and quota information for a storagebox.
#[derive(Debug, Clone, Deserialize)]
pub struct Disk {
    /// Storage quota.
    #[serde(rename = "disk_quota", with = "crate::bytes::mib")]
    pub quota: ByteSize,

    /// Storage usage in total (combined data & snapshots).
    #[serde(rename = "disk_usage", with = "crate::bytes::mib")]
    pub total: ByteSize,

    /// Storage used by data.
    #[serde(rename = "disk_usage_data", with = "crate::bytes::mib")]
    pub data: ByteSize,

    /// Storage used by snapshots.
    #[serde(rename = "disk_usage_snapshots", with = "crate::bytes::mib")]
    pub snapshots: ByteSize,
}

/// Services is an umbrella term covering the different features which might be
/// enabled on a storagebox.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Services {
    /// Indicates whether the storagebox is accessible via WebDAV.
    pub webdav: bool,

    /// Indicates whether the storagebox is available over Samba.
    pub samba: bool,

    /// Indicates whether the storagebox is accessible via SSH.
    pub ssh: bool,

    /// If enabled, a snapshots directory is mounted at from which
    /// data can be copied.
    ///
    /// See more at: <https://docs.hetzner.com/robot/storage-box/snapshots/>
    #[serde(rename = "zfs")]
    pub snapshot_directory: bool,

    /// Indicates whether the server is externally reachable.
    pub external_reachability: bool,
}

/// A snapshot is a point-in-time backup of the storagebox, which can be
/// used to restore the storagebox to the captured state..
#[derive(Debug, Clone, Deserialize)]
pub struct Snapshot {
    /// Name of the snapshot.
    pub name: String,

    /// Point in time at which the snapshot was taken.
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,

    /// Size of the snapshot.
    #[serde(with = "crate::bytes::mib")]
    pub size: ByteSize,

    /// Size of the filesystem.
    #[serde(with = "crate::bytes::mib")]
    pub filesystem_size: ByteSize,

    /// Indicates whether the snapshot was produced by an
    /// automatic or manual process.
    pub automatic: bool,

    /// Optional comment associated with the snapshot.
    pub comment: String,
}

/// Short summary of the newly created snapshot.
#[derive(Debug, Clone, Deserialize)]
pub struct CreatedSnapshot {
    /// Name of the snapshot.
    pub name: String,

    /// Point in time at which the snapshot was taken.
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,

    /// Size of the snapshot.
    #[serde(with = "crate::bytes::mib")]
    pub size: ByteSize,
}

/// Snapshot plans periodically take snapshots of the underlying storagebox.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SnapshotPlan {
    /// Indicates whether the snapshot plan is enabled or not.
    pub status: PlanStatus,

    /// Minute at which to take the snapshot.
    pub minute: Option<u8>,

    /// Hour at which to take the snapshot.
    pub hour: Option<u8>,

    /// Day of week on which to take snapshot.
    #[serde(default, with = "crate::timezones::weekday_plus_one")]
    pub day_of_week: Option<Weekday>,

    /// 1-indexed day of month on which to take a snapshot.
    pub day_of_month: Option<u8>,

    /// Month in which to take the snapshot.
    pub month: Option<Month>,

    /// Maximum number of snapshots to keep around for this plan.
    ///
    /// Stand-alone storageboxes are limited to 10 snapshots, while linked
    /// storageboxes are limited to only 2.
    pub max_snapshots: Option<u8>,
}

impl SnapshotPlan {
    /// Daily snapshots taken at the given time.
    pub fn daily(hour: u8, minute: u8) -> SnapshotPlan {
        SnapshotPlan {
            status: PlanStatus::Enabled,
            minute: Some(minute),
            hour: Some(hour),
            day_of_week: None,
            day_of_month: None,
            month: None,
            max_snapshots: None,
        }
    }

    /// Weekly snapshots taken on the given day of the week and time.
    pub fn weekly(day: Weekday, hour: u8, minute: u8) -> SnapshotPlan {
        SnapshotPlan {
            status: PlanStatus::Enabled,
            minute: Some(minute),
            hour: Some(hour),
            day_of_week: Some(day),
            day_of_month: None,
            month: None,
            max_snapshots: None,
        }
    }

    /// Monthly snapshots taken at the given day of the month and time.
    pub fn monthly(day: u8, hour: u8, minute: u8) -> SnapshotPlan {
        SnapshotPlan {
            status: PlanStatus::Enabled,
            minute: Some(minute),
            hour: Some(hour),
            day_of_week: None,
            day_of_month: Some(day),
            month: None,
            max_snapshots: None,
        }
    }

    /// Yearly snapshots, taken on the given day of the month and time.
    pub fn yearly(month: Month, day: u8, hour: u8, minute: u8) -> SnapshotPlan {
        SnapshotPlan {
            status: PlanStatus::Enabled,
            minute: Some(minute),
            hour: Some(hour),
            day_of_week: None,
            day_of_month: Some(day),
            month: Some(month),
            max_snapshots: None,
        }
    }

    /// Limit the maximum number of snapshots to keep.
    ///
    /// Stand-alone storageboxes are limited to 10 snapshots, while linked
    /// storageboxes are limited to only 2.
    pub fn with_limit(mut self, max_snapshots: u8) -> SnapshotPlan {
        self.max_snapshots = Some(max_snapshots);
        self
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PlanStatus {
    Enabled,
    Disabled,
}