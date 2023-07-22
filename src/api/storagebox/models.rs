use std::fmt::Display;

use bytesize::ByteSize;
use serde::{Deserialize, Serialize};
use time::Date;

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

    /// Disk usage and quota.
    #[serde(flatten)]
    pub disk: Disk,
}

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
