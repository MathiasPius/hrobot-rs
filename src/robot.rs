use reqwest::{blocking::Client, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Error;

pub trait SyncRobot {
    fn new(username: &str, password: &str) -> Self;
    fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error>;
    fn post<T: DeserializeOwned, U: Serialize>(&self, path: &str, form: U) -> Result<T, Error>;
    /// URL-encoding the [Firewall](`crate::Firewall`) configuration specifically is not possible using serde_urlencoding
    /// so we need this function for posting our manually serialized version
    fn post_raw<T: DeserializeOwned>(&self, path: &str, form: String) -> Result<T, Error>;
    fn put<T: DeserializeOwned, U: Serialize>(&self, path: &str, form: U) -> Result<T, Error>;
    fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error>;
}

pub struct Robot {
    client: Client,
    base_url: Url,
    basic_auth: (String, String),
}

impl SyncRobot for Robot {
    fn new(username: &str, password: &str) -> Robot {
        Robot {
            client: Client::new(),
            base_url: "https://robot-ws.your-server.de".parse().unwrap(),
            basic_auth: (username.to_string(), password.to_string()),
        }
    }

    fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        Ok(self
            .client
            .get(format!("{}{}", self.base_url, path))
            .basic_auth(&self.basic_auth.0, Some(&self.basic_auth.1))
            .send()?
            .json()?)
    }

    fn post<T: DeserializeOwned, U: Serialize>(&self, path: &str, form: U) -> Result<T, Error> {
        Ok(self
            .client
            .post(format!("{}{}", self.base_url, path))
            .basic_auth(&self.basic_auth.0, Some(&self.basic_auth.1))
            .form(&form)
            .send()?
            .json()?)
    }

    /// URL-encoding the [Firewall](`crate::Firewall`) configuration specifically is not possible using serde_urlencoding
    /// so we need this function for posting our manually serialized version
    fn post_raw<T: DeserializeOwned>(&self, path: &str, form: String) -> Result<T, Error> {
        Ok(self
            .client
            .post(format!("{}{}", self.base_url, path))
            .basic_auth(&self.basic_auth.0, Some(&self.basic_auth.1))
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(form)
            .send()?
            .json()?)
    }

    fn put<T: DeserializeOwned, U: Serialize>(&self, path: &str, form: U) -> Result<T, Error> {
        Ok(self
            .client
            .put(format!("{}{}", self.base_url, path))
            .basic_auth(&self.basic_auth.0, Some(&self.basic_auth.1))
            .form(&form)
            .send()?
            .json()?)
    }

    fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        Ok(self
            .client
            .delete(format!("{}{}", self.base_url, path))
            .basic_auth(&self.basic_auth.0, Some(&self.basic_auth.1))
            .send()?
            .json()?)
    }
}

#[cfg(test)]
impl Default for Robot {
    fn default() -> Self {
        dotenv::dotenv().ok();

        let username = std::env::var("HROBOT_USERNAME")
            .expect("Robot WebService username must be provided via HROBOT_USERNAME");
        let password = std::env::var("HROBOT_PASSWORD")
            .expect("Robot WebService password must be provided via HROBOT_PASSWORD");

        Robot::new(&username, &password)
    }
}
