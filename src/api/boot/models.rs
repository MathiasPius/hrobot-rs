use serde::{Deserialize, Serialize};

use crate::api::keys::Key;

/// Keyboard layout.
///
/// Defaults to US.
#[derive(Debug, Clone, Copy, Default, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keyboard {
    #[default]
    #[serde(rename = "us")]
    US,
    #[serde(rename = "uk")]
    UK,
    #[serde(rename = "ch")]
    Swiss,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "fi")]
    Finnish,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "jp")]
    Japanese,
}

#[derive(Debug, Default, Clone, Serialize, PartialEq, Eq)]
pub struct RescueConfig {
    /// Rescue operating system to activate.
    #[serde(rename = "os")]
    pub operating_system: String,

    /// Key fingerprints to authorize for server access.
    #[serde(rename = "authorized_key", skip_serializing_if = "Vec::is_empty")]
    pub authorized_keys: Vec<String>,

    /// Keyboard layout to use for the rescue system.
    ///
    /// Defaults to US.
    pub keyboard: Keyboard,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ActiveRescueConfig {
    /// Active rescue operating system.
    #[serde(rename = "os")]
    pub operating_system: String,

    /// Root password for the currently active rescue system.
    pub password: String,

    /// Rescue system host keys
    pub host_key: Vec<String>,

    /// Keys authorized to access the rescue system via SSH.
    pub authorized_key: Vec<Key>,
}

/// Available rescue system configurations
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AvailableRescueConfig {
    /// Available rescue operating systems.
    #[serde(rename = "os")]
    pub operating_systems: Vec<String>,
}

/// Represents the currently active rescue configuration,
/// or if inactive, the available rescue systems.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Rescue {
    /// Currently active rescue system
    Active(ActiveRescueConfig),

    /// Available rescue system configurations
    Available(AvailableRescueConfig),
}
