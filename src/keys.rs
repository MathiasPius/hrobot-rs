use serde::{Deserialize, Serialize};

use crate::{Error, Robot};

#[derive(Debug, Deserialize)]
pub struct Key {
    pub name: String,
    pub fingerprint: String,
    #[serde(rename = "type")]
    pub key_type: String,
    pub size: u32,
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct KeyResponse {
    pub key: Key,
}

impl From<KeyResponse> for Key {
    fn from(k: KeyResponse) -> Self {
        k.key
    }
}

pub trait KeyRobot {
    fn list_keys(&self) -> Result<Vec<Key>, Error>;
    fn add_key(&self, name: &str, openssh_key: &str) -> Result<Key, Error>;
    fn get_key(&self, fingerprint: &str) -> Result<Key, Error>;
    fn rename_key(&self, fingerprint: &str, name: &str) -> Result<Key, Error>;
    fn delete_key(&self, fingerprint: &str) -> Result<(), Error>;
}

impl KeyRobot for Robot {
    fn list_keys(&self) -> Result<Vec<Key>, Error> {
        self.get::<Vec<KeyResponse>>("/key")
            .map(|k| k.into_iter().map(Key::from).collect())
    }

    fn add_key(&self, name: &str, openssh_key: &str) -> Result<Key, Error> {
        #[derive(Serialize)]
        struct AddKeyRequest<'a> {
            pub name: &'a str,
            pub data: &'a str,
        }

        self.post::<KeyResponse, AddKeyRequest>(
            "/key",
            AddKeyRequest {
                name,
                data: openssh_key,
            },
        )
        .map(Key::from)
    }

    fn get_key(&self, fingerprint: &str) -> Result<Key, Error> {
        self.get::<KeyResponse>(&format!("/key/{}", fingerprint))
            .map(Key::from)
    }

    fn rename_key(&self, fingerprint: &str, name: &str) -> Result<Key, Error> {
        #[derive(Serialize)]
        struct RenameKeyRequest<'a> {
            pub name: &'a str,
        }

        self.post::<KeyResponse, RenameKeyRequest>(
            &format!("/key/{}", fingerprint),
            RenameKeyRequest { name },
        )
        .map(Key::from)
    }

    fn delete_key(&self, fingerprint: &str) -> Result<(), Error> {
        self.delete::<()>(&format!("/key/{}", fingerprint))
    }
}
