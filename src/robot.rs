use log::debug;
use reqwest::{blocking::Client, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{APIResult, Error};

pub trait SyncRobot {
    fn new(username: &str, password: &str) -> Self;
    fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error>;
    fn post<T: DeserializeOwned, U: Serialize>(&self, path: &str, form: U) -> Result<T, Error>;
    /// URL-encoding the [Firewall](`crate::Firewall`) configuration specifically is not possible using serde_urlencoding
    /// so we need this function for posting our manually serialized version
    fn post_raw<T: DeserializeOwned>(&self, path: &str, form: String) -> Result<T, Error>;
    fn put<T: DeserializeOwned, U: Serialize>(&self, path: &str, form: U) -> Result<T, Error>;
    fn delete<T: DeserializeOwned, U: Serialize>(
        &self,
        path: &str,
        parameters: U,
    ) -> Result<T, Error>;
    fn delete_raw<T: DeserializeOwned>(&self, path: &str, form: String) -> Result<T, Error>;
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
        let full_path = format!("{}{}", self.base_url, path);
        debug!("GET {}", full_path);

        let result: String = self
            .client
            .get(full_path)
            .basic_auth(&self.basic_auth.0, Some(&self.basic_auth.1))
            .send()?
            .text()?;

        debug!("API Response: {}", result);
        serde_json::from_str::<APIResult<T>>(&result)?.into()
    }

    fn post<T: DeserializeOwned, U: Serialize>(&self, path: &str, form: U) -> Result<T, Error> {
        let full_path = format!("{}{}", self.base_url, path);
        debug!("POST {}, {:#?}", full_path, serde_json::to_string(&form));

        let result: String = self
            .client
            .post(full_path)
            .basic_auth(&self.basic_auth.0, Some(&self.basic_auth.1))
            .form(&form)
            .send()?
            .text()?;

        debug!("API Response: {}", result);
        serde_json::from_str::<APIResult<T>>(&result)?.into()
    }

    /// URL-encoding the [Firewall](`crate::Firewall`) configuration specifically is not possible using serde_urlencoding
    /// so we need this function for posting our manually serialized version
    fn post_raw<T: DeserializeOwned>(&self, path: &str, form: String) -> Result<T, Error> {
        let full_path = format!("{}{}", self.base_url, path);
        debug!("POST {}, {}", full_path, form);

        let result: String = self
            .client
            .post(format!("{}{}", self.base_url, path))
            .basic_auth(&self.basic_auth.0, Some(&self.basic_auth.1))
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(form)
            .send()?
            .text()?;

        debug!("API Response: {}", result);
        serde_json::from_str::<APIResult<T>>(&result)?.into()
    }

    fn put<T: DeserializeOwned, U: Serialize>(&self, path: &str, form: U) -> Result<T, Error> {
        let full_path = format!("{}{}", self.base_url, path);
        debug!("PUT {}, {:#?}", full_path, serde_json::to_string(&form));

        let result: String = self
            .client
            .put(format!("{}{}", self.base_url, path))
            .basic_auth(&self.basic_auth.0, Some(&self.basic_auth.1))
            .form(&form)
            .send()?
            .text()?;

        debug!("API Response: {}", result);
        serde_json::from_str::<APIResult<T>>(&result)?.into()
    }

    fn delete<T: DeserializeOwned, U: Serialize>(
        &self,
        path: &str,
        parameters: U,
    ) -> Result<T, Error> {
        let full_path = format!("{}{}", self.base_url, path);
        debug!(
            "DELETE {}, {:#?}",
            full_path,
            serde_json::to_string(&parameters)
        );

        let result: String = self
            .client
            .delete(format!("{}{}", self.base_url, path))
            .basic_auth(&self.basic_auth.0, Some(&self.basic_auth.1))
            .form(&parameters)
            .send()?
            .text()?;

        serde_json::from_str::<APIResult<T>>(&result)?.into()
    }

    fn delete_raw<T: DeserializeOwned>(&self, path: &str, parameters: String) -> Result<T, Error> {
        let full_path = format!("{}{}", self.base_url, path);
        debug!("DELETE {}, {}", full_path, &parameters);

        let result: String = self
            .client
            .delete(format!("{}{}", self.base_url, path))
            .basic_auth(&self.basic_auth.0, Some(&self.basic_auth.1))
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(parameters)
            .send()?
            .text()?;

        debug!("API Response: {}", result);
        serde_json::from_str::<APIResult<T>>(&result)?.into()
    }
}

#[cfg(test)]
impl Default for Robot {
    fn default() -> Self {
        dotenv::dotenv().ok();
        env_logger::init();

        let username = std::env::var("HROBOT_USERNAME")
            .expect("Robot WebService username must be provided via HROBOT_USERNAME");
        let password = std::env::var("HROBOT_PASSWORD")
            .expect("Robot WebService password must be provided via HROBOT_PASSWORD");

        Robot::new(&username, &password)
    }
}
