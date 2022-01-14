use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct APIError {
    pub status: u32,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum APIResult<T> {
    Ok(T),
    Error(APIError),
}

impl<T> From<APIResult<T>> for Result<T, Error> {
    fn from(result: APIResult<T>) -> Self {
        match result {
            APIResult::Ok(inner) => Ok(inner),
            APIResult::Error(e) => Err(Error::API(e)),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    API(APIError),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<APIError> for Error {
    fn from(e: APIError) -> Self {
        Error::API(e)
    }
}
