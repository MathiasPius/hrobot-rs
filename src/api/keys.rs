use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{error::Error, AsyncHttpClient, AsyncRobot};

use super::{
    wrapper::{List, Single},
    UnauthenticatedRequest,
};

/// SSH Public Key
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct Key {
    /// Unique name for the key.
    pub name: String,

    /// Fingerprint of the public key.
    pub fingerprint: String,

    /// Key algorithm (ED25519, RSA)
    #[serde(rename = "type")]
    pub algorithm: String,

    /// Key bit size.
    #[serde(rename = "size")]
    pub bits: u16,

    /// OpenSSH-formatted Key
    pub data: String,

    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
}

/// SSH Public Key Reference.
///
/// This is just key metadata, it does not contain the key itself.
///
/// To retrieve the key, see [`AsyncRobot::get_ssh_key`].
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct KeyReference {
    /// Unique name for the key.
    pub name: String,

    /// Fingerprint of the public key.
    pub fingerprint: String,

    /// Key algorithm (ED25519, RSA)
    #[serde(rename = "type")]
    pub algorithm: String,

    /// Key bit size.
    #[serde(rename = "size")]
    pub bits: u16,

    #[serde(deserialize_with = "crate::timezones::assume_berlin_timezone")]
    pub created_at: OffsetDateTime,
}

fn list_ssh_keys() -> UnauthenticatedRequest<List<Key>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/key")
}

fn create_ssh_key(
    name: &str,
    key: &str,
) -> Result<UnauthenticatedRequest<Single<Key>>, serde_html_form::ser::Error> {
    #[derive(Serialize)]
    struct CreateSshKey<'a> {
        name: &'a str,
        data: &'a str,
    }

    UnauthenticatedRequest::from("https://robot-ws.your-server.de/key")
        .with_method("POST")
        .with_body(CreateSshKey { name, data: key })
}

fn get_ssh_key(fingerprint: &str) -> UnauthenticatedRequest<Single<Key>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/key/{fingerprint}"
    ))
}

fn remove_ssh_key(fingerprint: &str) -> UnauthenticatedRequest<()> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/key/{fingerprint}"
    ))
    .with_method("DELETE")
}

fn rename_ssh_key(
    fingerprint: &str,
    new_name: &str,
) -> Result<UnauthenticatedRequest<Single<Key>>, serde_html_form::ser::Error> {
    #[derive(Serialize)]
    struct RenameSshKey<'a> {
        name: &'a str,
    }

    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/key/{fingerprint}"
    ))
    .with_method("POST")
    .with_body(RenameSshKey { name: new_name })
}

impl<Client: AsyncHttpClient> AsyncRobot<Client> {
    /// List all SSH [`Key`]s.
    ///
    /// # Example
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// for key in robot.list_ssh_keys().await.unwrap() {
    ///     println!("{}: {}", key.name, key.fingerprint)
    /// }
    /// # }
    /// ```
    pub async fn list_ssh_keys(&self) -> Result<Vec<Key>, Error> {
        Ok(self.go(list_ssh_keys()).await?.0)
    }

    /// Retrieve a single SSH [`Key`].
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// let key = robot.get_ssh_key("d7:34:1c:8c:4e:20:e0:1f:07:66:45:d9:97:22:ec:07").await.unwrap();
    ///
    /// println!("{key:#?}");
    /// # }
    /// ```
    pub async fn get_ssh_key(&self, fingerprint: &str) -> Result<Key, Error> {
        Ok(self.go(get_ssh_key(fingerprint)).await?.0)
    }

    /// Upload a new SSH [`Key`].
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// let key = robot.create_ssh_key(
    ///     "hrobot-rs-test-key",
    ///     "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIEaQde8iCKizUOiXlowY1iEL1yCufgjb3aiatGQNPcHb"
    /// ).await.unwrap();
    ///
    /// println!("{key:#?}");
    /// # }
    /// ```
    pub async fn create_ssh_key(&self, name: &str, key: &str) -> Result<Key, Error> {
        Ok(self.go(create_ssh_key(name, key)?).await?.0)
    }

    /// Remove an SSH [`Key`].
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.remove_ssh_key(
    ///     "d7:34:1c:8c:4e:20:e0:1f:07:66:45:d9:97:22:ec:07"
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn remove_ssh_key(&self, fingerprint: &str) -> Result<(), Error> {
        self.go(remove_ssh_key(fingerprint)).await.or_else(|err| {
            // Recover from error caused by attempting to deserialize ().
            if matches!(err, Error::Deserialization(_)) {
                Ok(())
            } else {
                Err(err)
            }
        })
    }

    /// Rename an SSH [`Key`].
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.rename_ssh_key(
    ///     "d7:34:1c:8c:4e:20:e0:1f:07:66:45:d9:97:22:ec:07",
    ///     "new-name"
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn rename_ssh_key(&self, fingerprint: &str, new_name: &str) -> Result<Key, Error> {
        Ok(self.go(rename_ssh_key(fingerprint, new_name)?).await?.0)
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use time::macros::datetime;
    use tracing::info;
    use tracing_test::traced_test;

    use crate::api::keys::KeyReference;

    #[test]
    fn test_key_deserialization() {
        let key = r#"
            {
                "name": "hrobot-rs-test-key",
                "fingerprint":"d7:34:1c:8c:4e:20:e0:1f:07:66:45:d9:97:22:ec:07",
                "type":"ED25519",
                "size":256,
                "created_at":"2023-06-10 21:34:12"
            }
        "#;

        assert_eq!(
            KeyReference {
                name: "hrobot-rs-test-key".to_string(),
                fingerprint: "d7:34:1c:8c:4e:20:e0:1f:07:66:45:d9:97:22:ec:07".to_string(),
                algorithm: "ED25519".to_string(),
                bits: 256,
                created_at: datetime!(2023-06-10 21:34:12 +02:00)
            },
            serde_json::from_str(key).unwrap()
        )
    }

    #[tokio::test]
    #[traced_test]
    #[ignore = "unexpected failure might leave test key behind."]
    #[serial("ssh-keys")]
    async fn test_create_delete_key() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

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
        robot
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
}
