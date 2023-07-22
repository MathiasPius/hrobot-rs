use crate::{error::Error, AsyncRobot};

use super::{
    wrapper::{List, Single},
    UnauthenticatedRequest,
};

mod models;
pub use models::*;

fn list_storageboxes() -> UnauthenticatedRequest<List<StorageBoxReference>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/storagebox")
}

fn get_storagebox(storagebox: StorageBoxId) -> UnauthenticatedRequest<Single<StorageBox>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/storagebox/{storagebox}"
    ))
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
    /// ```rust
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
}

#[cfg(test)]
mod tests {
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

        if let Some(storagebox) = storageboxes.first() {
            let storagebox = robot.get_storagebox(storagebox.id).await.unwrap();
            info!("{storagebox:#?}");
        }
    }
}
