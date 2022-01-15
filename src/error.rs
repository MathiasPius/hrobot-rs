use serde::Deserialize;

#[derive(Debug)]
pub enum APIError {
    Unavailable,
    ServerNotFound {
        message: String,
    },
    RateLimitExceeded {
        max_request: u32,
        interval: u32,
    },
    InvalidInput {
        missing: Vec<String>,
        invalid: Vec<String>,
    },
    Generic {
        status: u32,
        code: String,
        message: String,
    },
}

#[derive(Debug, Deserialize)]
struct InvalidInputError {
    #[serde(default)]
    pub missing: Vec<String>,
    #[serde(default)]
    pub invalid: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RateLimitError {
    pub interval: u32,
    pub max_request: u32,
}

#[derive(Debug, Deserialize)]
struct GenericAPIError {
    pub status: u32,
    pub code: String,
    pub message: String,
    #[serde(flatten)]
    pub invalid_input: Option<InvalidInputError>,
    #[serde(flatten)]
    pub rate_limit: Option<RateLimitError>,
}

impl From<GenericAPIError> for APIError {
    fn from(err: GenericAPIError) -> Self {
        match (err.status, err.code.as_str()) {
            (503, _) => APIError::Unavailable,
            (403, "RATE_LIMIT_EXCEEDED") => {
                let rate_limit = err.rate_limit.expect("API returned RATE_LIMIT_EXCEEDED error, but did not include information about max_requests or intervals");

                APIError::RateLimitExceeded {
                    max_request: rate_limit.max_request,
                    interval: rate_limit.interval,
                }
            }
            (_, "INVALID_INPUT") => {
                let invalid_input = err.invalid_input.expect("API returned INVALID_INPUT error, but did not include any information regarding missing input");

                APIError::InvalidInput {
                    missing: invalid_input.missing,
                    invalid: invalid_input.invalid,
                }
            }
            (_, "SERVER_NOT_FOUND") => APIError::ServerNotFound {
                message: err.message,
            },
            _ => APIError::Generic {
                status: err.status,
                code: err.code,
                message: err.message,
            },
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum APIResult<T> {
    Ok(T),
    Error(GenericAPIError),
}

impl<T> From<APIResult<T>> for Result<T, Error> {
    fn from(result: APIResult<T>) -> Self {
        match result {
            APIResult::Ok(inner) => Ok(inner),
            APIResult::Error(e) => Err(Error::API(APIError::from(e))),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Transport(Box<dyn std::error::Error>),
    API(APIError),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Transport(Box::new(e))
    }
}

impl From<APIError> for Error {
    fn from(e: APIError) -> Self {
        Error::API(e)
    }
}
