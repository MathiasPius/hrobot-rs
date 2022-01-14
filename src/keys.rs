use serde::{Deserialize, Serialize};

use crate::{APIResult, Error, Robot};

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

pub trait KeyRobot {
    fn list_keys(&self) -> Result<Vec<Key>, Error>;
    fn add_key(&self, name: &str, openssh_key: &str) -> Result<Key, Error>;
    fn get_key(&self, fingerprint: &str) -> Result<Key, Error>;
    fn rename_key(&self, fingerprint: &str, name: &str) -> Result<Key, Error>;
    fn delete_key(&self, fingerprint: &str) -> Result<(), Error>;
}

impl KeyRobot for Robot {
    fn list_keys(&self) -> Result<Vec<Key>, Error> {
        let result: APIResult<Vec<KeyResponse>> = self.get("/key")?;

        match result {
            APIResult::Ok(s) => Ok(s.into_iter().map(|s| s.key).collect()),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn add_key(&self, name: &str, openssh_key: &str) -> Result<Key, Error> {
        #[derive(Serialize)]
        struct AddKeyRequest<'a> {
            pub name: &'a str,
            pub data: &'a str,
        }

        let result: APIResult<KeyResponse> = self.post(
            "/key",
            AddKeyRequest {
                name,
                data: openssh_key,
            },
        )?;

        match result {
            APIResult::Ok(s) => Ok(s.key),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn get_key(&self, fingerprint: &str) -> Result<Key, Error> {
        let result: APIResult<KeyResponse> = self.get(&format!("/key/{}", fingerprint))?;

        match result {
            APIResult::Ok(s) => Ok(s.key),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn rename_key(&self, fingerprint: &str, name: &str) -> Result<Key, Error> {
        #[derive(Serialize)]
        struct RenameKeyRequest<'a> {
            pub name: &'a str,
        }

        let result: APIResult<KeyResponse> =
            self.post(&format!("/key/{}", fingerprint), RenameKeyRequest { name })?;

        match result {
            APIResult::Ok(s) => Ok(s.key),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn delete_key(&self, fingerprint: &str) -> Result<(), Error> {
        let result: APIResult<()> = self.delete(&format!("/key/{}", fingerprint))?;

        match result {
            APIResult::Ok(_) => Ok(()),
            APIResult::Error(e) => Err(e.into()),
        }
    }
}
