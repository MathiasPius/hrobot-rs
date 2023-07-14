//! Data model for the API.
//!
//! Contains all the object types used in both requests
//! and responses by the API.

pub(crate) mod firewall;
pub(crate) mod server;
pub use firewall::*;
pub use server::*;
