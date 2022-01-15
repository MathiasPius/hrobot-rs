use serde::Deserialize;

#[derive(Debug)]
pub enum APIError {
    Unavailable,
    NotFound {
        message: String,
    },
    ServerNotFound {
        message: String,
    },
    IPNotFound {
        message: String,
    },
    SubnetNotFound {
        message: String,
    },
    MACNotFound {
        message: String,
    },
    MACNotAvailable {
        message: String,
    },
    MACAlreadySet {
        message: String,
    },
    MACFailed {
        message: String,
    },
    WOLNotAvailable {
        message: String,
    },
    WOLFailed {
        message: String,
    },
    RateLimitExceeded {
        message: String,
        max_request: u32,
        interval: u32,
    },
    ResetNotAvailable {
        message: String,
    },
    ManualResetActive {
        message: String,
    },
    ResetFailed {
        message: String,
    },
    InvalidInput {
        message: String,
        missing: Vec<String>,
        invalid: Vec<String>,
    },
    Conflict {
        message: String,
    },
    ServerReservationFailed {
        message: String,
    },
    TrafficWarningUpdateFailed {
        message: String,
    },
    BootNotAvailable {
        message: String,
    },
    InternalError {
        message: String,
    },
    FailoverAlreadyRouted {
        message: String,
    },
    FailoverFailed {
        message: String,
    },
    FailoverLocked {
        message: String,
    },
    FailoverNotComplete {
        message: String,
    },
    WithdrawalFailed {
        message: String,
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
            (403, "RATE_LIMIT_EXCEEDED") => {
                let rate_limit = err.rate_limit.expect("API returned RATE_LIMIT_EXCEEDED error, but did not include information about max_requests or intervals");

                APIError::RateLimitExceeded {
                    message: err.message,
                    max_request: rate_limit.max_request,
                    interval: rate_limit.interval,
                }
            }
            (400, "INVALID_INPUT") => {
                let invalid_input = err.invalid_input.expect("API returned INVALID_INPUT error, but did not include any information regarding missing input");

                APIError::InvalidInput {
                    message: err.message,
                    missing: invalid_input.missing,
                    invalid: invalid_input.invalid,
                }
            },
            (409, "CONFLICT") => APIError::Conflict {
                message: err.message
            },
            (404, "NOT_FOUND") => APIError::NotFound {
                message: err.message,
            },
            (404, "SERVER_NOT_FOUND") => APIError::ServerNotFound {
                message: err.message,
            },
            (404, "IP_NOT_FOUND") => APIError::IPNotFound {
                message: err.message,
            },
            (404, "SUBNET_NOT_FOUND") => APIError::SubnetNotFound {
                message: err.message,
            },
            (404, "MAC_NOT_FOUND") => APIError::MACNotFound {
                message: err.message,
            },
            (404, "MAC_NOT_AVAILABLE") => APIError::MACNotAvailable {
                message: err.message,
            },
            (404, "FAILOVER_NEW_SERVER_NOT_FOUND") => APIError::ServerNotFound {
                message: err.message,
            },
            (404, "WOL_NOT_AVAILABLE") => APIError::WOLNotAvailable {
                message: err.message,
            },
            (404, "BOOT_NOT_AVAILABLE") => APIError::BootNotAvailable {
                message: err.message,
            },
            (409, "FAILOVER_LOCKED") => APIError::FailoverLocked {
                message: err.message,
            },
            (409, "FAILOVER_ALREADY_ROUTED") => APIError::FailoverAlreadyRouted {
                message: err.message,
            },
            (409, "RESET_MANUAL_ACTIVE") => APIError::ManualResetActive {
                message: err.message,
            },
            (409, "MAC_ALREADY_SET") => APIError::MACAlreadySet{
                message: err.message,
            },
            (409, "SERVER_CANCELLATION_RESERVE_LOCATION_FALSE_ONLY") => APIError::ServerReservationFailed {
                message: err.message,
            },
            (409, "SERVER_REVERSAL_NOT_POSSIBLE") => APIError::WithdrawalFailed {
                message: err.message,
            },
            (500, "INTERNAL_ERROR") => APIError::InternalError {
                message: err.message,
            },
            (500, "MAC_FAILED") => APIError::MACFailed {
                message: err.message,
            },
            (500, "WOL_FAILED") => APIError::WOLFailed {
                message: err.message,
            },
            (500, "RESET_FAILED") => APIError::ResetFailed {
                message: err.message,
            },
            (500, "TRAFFIC_WARNING_UPDATE_FAILED") => APIError::TrafficWarningUpdateFailed {
                message: err.message
            },
            (500, "FAILOVER_FAILED") => APIError::FailoverFailed {
                message: err.message,
            },
            (500, "FAILOVER_NOT_COMPLETE") => APIError::FailoverNotComplete {
                message: err.message,
            },
            (503, _) => APIError::Unavailable,
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
