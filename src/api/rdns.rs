use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::{error::Error, AsyncHttpClient, AsyncRobot};

use super::{
    wrapper::{List, Single},
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

fn delete_rdns_entry(ip: IpAddr) -> UnauthenticatedRequest<()> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/rdns/{ip}"))
        .with_method("DELETE")
}

impl<Client: AsyncHttpClient> AsyncRobot<Client> {
    /// List all Reverse DNS entries.
    ///
    /// # Example
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
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
    pub async fn create_rdns_entry(&self, ip: IpAddr, ptr: &str) -> Result<(), Error> {
        self.go(create_rdns_entry(ip, ptr)?).await?;
        Ok(())
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
    pub async fn update_rdns_entry(&self, ip: IpAddr, ptr: &str) -> Result<(), Error> {
        self.go(update_rdns_entry(ip, ptr)?).await?;
        Ok(())
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
        self.go(delete_rdns_entry(ip)).await.or_else(|err| {
            // Recover from error caused by attempting to deserialize ().
            if matches!(err, Error::Deserialization(_)) {
                Ok(())
            } else {
                Err(err)
            }
        })
    }
}

/// Reverse DNS Entry.
///
/// Maps an IP address to a single domain.
#[derive(Debug, Clone, Deserialize)]
pub struct RdnsEntry {
    /// IP Address this entry represents.
    pub ip: IpAddr,
    
    /// The target domain/record.
    pub ptr: String,
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    use crate::error::{ApiError, Error};

    #[tokio::test]
    #[traced_test]
    async fn test_list_rdns() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let rdns_entries = robot.list_rdns_entries().await.unwrap();
        info!("{rdns_entries:#?}");
    }

    #[tokio::test]
    #[traced_test]
    async fn test_get_rdns() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let rdns_entries = robot.list_rdns_entries().await.unwrap();
        info!("{rdns_entries:#?}");

        if let Some(entry) = rdns_entries.first() {
            let rdns = robot.get_rdns_entry(entry.ip).await.unwrap();

            info!("{rdns}");
        }
    }

    #[tokio::test]
    #[traced_test]
    #[ignore = "unexpected failure can leave rdns entries intact"]
    async fn test_create_update_delete_rdns() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let subnets = robot.list_subnets().await.unwrap();
        info!("{subnets:#?}");

        let ip = subnets
            .values()
            .into_iter()
            .filter_map(|subnet| subnet.first())
            .find_map(|subnet| {
                if subnet.ip.addr().is_ipv6() {
                    Some(subnet.ip.addr())
                } else {
                    None
                }
            });

        if let Some(ip) = ip {
            assert!(matches!(
                robot.get_rdns_entry(ip).await,
                Err(Error::Api(ApiError::RdnsNotFound { .. }))
            ));

            robot
                .create_rdns_entry(ip, "test.example.com")
                .await
                .unwrap();

            assert_eq!(robot.get_rdns_entry(ip).await.unwrap(), "test.example.com");

            robot
                .update_rdns_entry(ip, "test2.example.com")
                .await
                .unwrap();

            assert_eq!(robot.get_rdns_entry(ip).await.unwrap(), "test2.example.com");

            robot.delete_rdns_entry(ip).await.unwrap();
        }
    }
}
