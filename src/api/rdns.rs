//! Reverse DNS structs and implementations.

use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::{error::Error, AsyncRobot};

use super::{
    wrapper::{Empty, List, Single},
    UnauthenticatedRequest,
};

fn list_rdns_entries() -> UnauthenticatedRequest<List<RdnsEntry>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/rdns")
}

fn get_rdns_entry(ip: IpAddr) -> UnauthenticatedRequest<Single<RdnsEntry>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/rdns/{ip}"))
}

#[derive(Serialize)]
struct SetPtr<'a> {
    ptr: &'a str,
}

fn create_rdns_entry(
    ip: IpAddr,
    ptr: &str,
) -> Result<UnauthenticatedRequest<Single<RdnsEntry>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/rdns/{ip}"))
        .with_method("PUT")
        .with_body(SetPtr { ptr })
}

fn update_rdns_entry(
    ip: IpAddr,
    ptr: &str,
) -> Result<UnauthenticatedRequest<Single<RdnsEntry>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/rdns/{ip}"))
        .with_method("POST")
        .with_body(SetPtr { ptr })
}

fn delete_rdns_entry(ip: IpAddr) -> UnauthenticatedRequest<Empty> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/rdns/{ip}"))
        .with_method("DELETE")
}

impl AsyncRobot {
    /// List all Reverse DNS entries.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.list_rdns_entries().await.unwrap();
    /// # }
    /// ```
    pub async fn list_rdns_entries(&self) -> Result<Vec<RdnsEntry>, Error> {
        Ok(self.go(list_rdns_entries()).await?.0)
    }

    /// Get Reverse DNS entry for IP address.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_rdns_entry("123.123.123.123".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn get_rdns_entry(&self, ip: IpAddr) -> Result<String, Error> {
        Ok(self.go(get_rdns_entry(ip)).await?.0.ptr)
    }

    /// Create Reverse DNS entry for IP address.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.create_rdns_entry("123.123.123.123".parse().unwrap(), "test.example.com").await.unwrap();
    /// # }
    /// ```
    pub async fn create_rdns_entry(&self, ip: IpAddr, ptr: &str) -> Result<RdnsEntry, Error> {
        Ok(self.go(create_rdns_entry(ip, ptr)?).await?.0)
    }

    /// Update Reverse DNS entry for IP address.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.update_rdns_entry("123.123.123.123".parse().unwrap(), "test.example.com").await.unwrap();
    /// # }
    /// ```
    pub async fn update_rdns_entry(&self, ip: IpAddr, ptr: &str) -> Result<RdnsEntry, Error> {
        Ok(self.go(update_rdns_entry(ip, ptr)?).await?.0)
    }

    /// Delete Reverse DNS entry for IP address.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.delete_rdns_entry("123.123.123.123".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn delete_rdns_entry(&self, ip: IpAddr) -> Result<(), Error> {
        self.go(delete_rdns_entry(ip)).await?.throw_away();
        Ok(())
    }
}

/// Reverse DNS Entry.
///
/// Maps an IP address to a single domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdnsEntry {
    /// IP Address this entry represents.
    pub ip: IpAddr,

    /// The target domain/record.
    pub ptr: String,
}
