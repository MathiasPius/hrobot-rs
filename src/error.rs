use serde::Deserialize;

#[derive(Debug)]
pub enum Addon {
    Windows,
    Plesk,
    CPanel,
}

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
    WindowsOutdated {
        message: String,
    },
    MissingAddon {
        addon: Addon,
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
    StorageboxNotFound {
        message: String,
    },
    StorageboxSubaccountNotFound {
        message: String,
    },
    SnapshotNotFound {
        message: String,
    },
    SnapshotLimitExceeded {
        message: String,
    },
    FirewallPortNotFound {
        message: String,
    },
    FirewallNotAvailable {
        message: String,
    },
    FirewallTemplateNotFound {
        message: String,
    },
    FirewallInProcess {
        message: String,
    },
    VSwitchLimitReached {
        message: String,
    },
    VSwitchServerLimitReached {
        message: String,
    },
    VSwitchPerServerLimitReached {
        message: String,
    },
    VSwitchInProcess {
        message: String,
    },
    VSwitchVlanNotUnique {
        message: String,
    },
    ManualResetActive {
        message: String,
    },
    KeyUpdateFailed {
        message: String,
    },
    KeyCreateFailed {
        message: String,
    },
    KeyDeleteFailed {
        message: String,
    },
    KeyAlreadyExists {
        message: String,
    },
    RDNSNotFound {
        message: String,
    },
    RDNSCreateFailed {
        message: String,
    },
    RDNSUpdateFailed {
        message: String,
    },
    RDNSDeleteFailed {
        message: String,
    },
    RDNSAlreadyExists {
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
    BootActivationFailed {
        message: String,
    },
    BootDeactivationFailed {
        message: String,
    },
    BootAlreadyEnabled {
        message: String,
    },
    BootBlocked {
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
            }
            (409, "CONFLICT") => APIError::Conflict {
                message: err.message,
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
            (404, "RDNS_NOT_FOUND") => APIError::RDNSNotFound {
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
            (404, "WINDOWS_OUTDATED_VERSION") => APIError::WindowsOutdated {
                message: err.message,
            },
            (404, "WINDOWS_MISSING_ADDON") => APIError::MissingAddon {
                addon: Addon::Windows,
                message: err.message,
            },
            (404, "PLESK_MISSING_ADDON") => APIError::MissingAddon {
                addon: Addon::Plesk,
                message: err.message,
            },
            (404, "CPANEL_MISSING_ADDON") => APIError::MissingAddon {
                addon: Addon::CPanel,
                message: err.message,
            },
            (404, "STORAGEBOX_NOT_FOUND") => APIError::StorageboxNotFound {
                message: err.message,
            },
            (404, "STORAGEBOX_SUBACCOUNT_NOT_FOUND") => APIError::StorageboxSubaccountNotFound {
                message: err.message,
            },
            (404, "SNAPSHOT_NOT_FOUND") => APIError::SnapshotNotFound {
                message: err.message,
            },
            (404, "FIREWALL_PORT_NOT_FOUND") => APIError::FirewallPortNotFound {
                message: err.message,
            },
            (404, "FIREWALL_NOT_AVAILABLE") => APIError::FirewallNotAvailable {
                message: err.message,
            },
            (404, "FIREWALL_TEMPLATE_NOT_FOUND") => APIError::FirewallTemplateNotFound {
                message: err.message,
            },
            (409, "FIREWALL_IN_PROCESS") => APIError::FirewallInProcess {
                message: err.message,
            },
            (409, "VSWITCH_LIMIT_REACHED") => APIError::VSwitchLimitReached {
                message: err.message,
            },
            (409, "VSWITCH_SERVER_LIMIT_REACHED") => APIError::VSwitchServerLimitReached {
                message: err.message,
            },
            (409, "VSWITCH_PER_SERVER_LIMIT_REACHED") => APIError::VSwitchPerServerLimitReached {
                message: err.message,
            },
            (409, "VSWITCH_IN_PROCESS") => APIError::VSwitchInProcess {
                message: err.message,
            },
            (409, "VSWITCH_VLAN_NOT_UNIQUE") => APIError::VSwitchVlanNotUnique {
                message: err.message,
            },
            (409, "RDNS_ALREADY_EXISTS") => APIError::RDNSAlreadyExists {
                message: err.message,
            },
            (409, "FAILOVER_LOCKED") => APIError::FailoverLocked {
                message: err.message,
            },
            (409, "SNAPSHOT_LIMIT_EXCEEDED") => APIError::SnapshotLimitExceeded {
                message: err.message,
            },
            (409, "FAILOVER_ALREADY_ROUTED") => APIError::FailoverAlreadyRouted {
                message: err.message,
            },
            (409, "RESET_MANUAL_ACTIVE") => APIError::ManualResetActive {
                message: err.message,
            },
            (409, "MAC_ALREADY_SET") => APIError::MACAlreadySet {
                message: err.message,
            },
            (409, "KEY_ALREADY_EXISTS") => APIError::KeyAlreadyExists {
                message: err.message,
            },
            (409, "SERVER_CANCELLATION_RESERVE_LOCATION_FALSE_ONLY") => {
                APIError::ServerReservationFailed {
                    message: err.message,
                }
            }
            (409, "SERVER_REVERSAL_NOT_POSSIBLE") => APIError::WithdrawalFailed {
                message: err.message,
            },
            (500, "KEY_UPDATE_FAILED") => APIError::KeyUpdateFailed {
                message: err.message,
            },
            (500, "KEY_CREATE_FAILED") => APIError::KeyCreateFailed {
                message: err.message,
            },
            (500, "KEY_DELETE_FAILED") => APIError::KeyDeleteFailed {
                message: err.message,
            },
            (500, "RNDS_CREATE_FAILED") => APIError::RDNSCreateFailed {
                message: err.message,
            },
            (500, "RNDS_UPDATE_FAILED") => APIError::RDNSUpdateFailed {
                message: err.message,
            },
            (500, "RNDS_DELETE_FAILED") => APIError::RDNSDeleteFailed {
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
                message: err.message,
            },
            (500, "FAILOVER_FAILED") => APIError::FailoverFailed {
                message: err.message,
            },
            (500, "FAILOVER_NOT_COMPLETE") => APIError::FailoverNotComplete {
                message: err.message,
            },
            (409, "BOOT_ALREADY_ENABLED") => APIError::BootAlreadyEnabled {
                message: err.message,
            },
            (409, "BOOT_BLOCKED") => APIError::BootBlocked {
                message: err.message,
            },
            (500, "BOOT_ACTIVATION_FAILED") => APIError::BootActivationFailed {
                message: err.message,
            },
            (500, "BOOT_DEACTIVATION_FAILED") => APIError::BootDeactivationFailed {
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
