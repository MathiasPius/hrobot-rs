use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "code", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum APIError {
    Unavailable,
    NotFound {
        message: String,
    },
    ServerNotFound {
        message: String,
    },
    IpNotFound {
        message: String,
    },
    SubnetNotFound {
        message: String,
    },
    MacNotFound {
        message: String,
    },
    MacNotAvailable {
        message: String,
    },
    MacAlreadySet {
        message: String,
    },
    MacFailed {
        message: String,
    },
    WolNotAvailable {
        message: String,
    },
    WolFailed {
        message: String,
    },
    WindowsOutdatedVersion {
        message: String,
    },
    WindowsMissingAddon {
        message: String,
    },
    PleskMissingAddon {
        message: String,
    },
    CpanelMissingAddon {
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
    StorageboxSubaccountLimitExceeded {
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
    VswitchLimitReached {
        message: String,
    },
    VswitchNotAvailable {
        message: String,
    },
    VswitchServerLimitReached {
        message: String,
    },
    VswitchPerServerLimitReached {
        message: String,
    },
    VswitchInProcess {
        message: String,
    },
    VswitchVlanNotUnique {
        message: String,
    },
    ResetManualActive {
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
    RdnsNotFound {
        message: String,
    },
    RdnsCreateFailed {
        message: String,
    },
    RdnsUpdateFailed {
        message: String,
    },
    RdnsDeleteFailed {
        message: String,
    },
    RdnsAlreadyExists {
        message: String,
    },
    ResetFailed {
        message: String,
    },
    InvalidInput {
        message: String,
        #[serde(default)]
        missing: Vec<String>,
        #[serde(default)]
        invalid: Vec<String>,
    },
    Conflict {
        message: String,
    },
    ServerCancellationReserveLocationFalseOnly {
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
    FailoverNewServerNotFound {
        message: String,
    },
    ServerReversalNotPossible {
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
    #[serde(skip_deserializing)]
    Generic(GenericError),
}

#[derive(Debug, Deserialize)]
pub struct InvalidInputError {
    #[serde(default)]
    pub missing: Vec<String>,
    #[serde(default)]
    pub invalid: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RateLimitError {
    pub interval: u32,
    pub max_request: u32,
}

#[derive(Debug, Deserialize)]
pub struct GenericError {
    pub status: u32,
    pub code: String,
    pub message: String,
    #[serde(flatten)]
    pub invalid_input: Option<InvalidInputError>,
    #[serde(flatten)]
    pub rate_limit: Option<RateLimitError>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MaybeTyped {
    Typed(APIError),
    Untyped(GenericError),
}

#[cfg(test)]
impl MaybeTyped {
    pub fn is_typed(&self) -> bool {
        match self {
            MaybeTyped::Typed(_) => true,
            MaybeTyped::Untyped(_) => false,
        }
    }
}

impl From<MaybeTyped> for APIError {
    fn from(maybe: MaybeTyped) -> Self {
        match maybe {
            MaybeTyped::Typed(t) => t,
            MaybeTyped::Untyped(t) => APIError::Generic(t),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum APIResult<T> {
    Ok(T),
    Error(MaybeTyped),
}

impl<T> From<APIResult<T>> for Result<T, Error> {
    fn from(result: APIResult<T>) -> Self {
        match result {
            APIResult::Ok(inner) => Ok(inner),
            APIResult::Error(e) => Err(Error::API(e.into())),
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    // Fetched using:
    // curl -s https://robot.your-server.de/doc/webservice/en.html | rg -r '"$1",' '.*<td>([A-Z][A-Z_]*)</td>.*' | sort | uniq
    const ERROR_CODES: &[&str] = &[
        "BOOT_ACTIVATION_FAILED",
        "BOOT_ALREADY_ENABLED",
        "BOOT_BLOCKED",
        "BOOT_DEACTIVATION_FAILED",
        "BOOT_NOT_AVAILABLE",
        "CONFLICT",
        "CPANEL_MISSING_ADDON",
        "FAILOVER_ALREADY_ROUTED",
        "FAILOVER_FAILED",
        "FAILOVER_LOCKED",
        "FAILOVER_NEW_SERVER_NOT_FOUND",
        "FAILOVER_NOT_COMPLETE",
        "FIREWALL_IN_PROCESS",
        "FIREWALL_NOT_AVAILABLE",
        "FIREWALL_PORT_NOT_FOUND",
        "FIREWALL_TEMPLATE_NOT_FOUND",
        "INTERNAL_ERROR",
        "INVALID_INPUT",
        "IP_NOT_FOUND",
        "KEY_ALREADY_EXISTS",
        "KEY_CREATE_FAILED",
        "KEY_DELETE_FAILED",
        "KEY_UPDATE_FAILED",
        "MAC_ALREADY_SET",
        "MAC_FAILED",
        "MAC_NOT_AVAILABLE",
        "MAC_NOT_FOUND",
        "NOT_FOUND",
        "PLESK_MISSING_ADDON",
        // This one requires additional fields,
        // so we can't test conversion generically
        //"RATE_LIMIT_EXCEEDED",
        "RDNS_ALREADY_EXISTS",
        "RDNS_CREATE_FAILED",
        "RDNS_DELETE_FAILED",
        "RDNS_NOT_FOUND",
        "RDNS_UPDATE_FAILED",
        "RESET_FAILED",
        "RESET_MANUAL_ACTIVE",
        "RESET_NOT_AVAILABLE",
        "SERVER_CANCELLATION_RESERVE_LOCATION_FALSE_ONLY",
        "SERVER_NOT_FOUND",
        "SERVER_REVERSAL_NOT_POSSIBLE",
        "SNAPSHOT_LIMIT_EXCEEDED",
        "SNAPSHOT_NOT_FOUND",
        "STORAGEBOX_NOT_FOUND",
        "STORAGEBOX_SUBACCOUNT_LIMIT_EXCEEDED",
        "STORAGEBOX_SUBACCOUNT_NOT_FOUND",
        "SUBNET_NOT_FOUND",
        "TRAFFIC_WARNING_UPDATE_FAILED",
        "VSWITCH_IN_PROCESS",
        "VSWITCH_LIMIT_REACHED",
        "VSWITCH_NOT_AVAILABLE",
        "VSWITCH_PER_SERVER_LIMIT_REACHED",
        "VSWITCH_SERVER_LIMIT_REACHED",
        "VSWITCH_VLAN_NOT_UNIQUE",
        "WINDOWS_MISSING_ADDON",
        "WINDOWS_OUTDATED_VERSION",
        "WOL_FAILED",
        "WOL_NOT_AVAILABLE",
    ];

    #[derive(Serialize)]
    struct ErrorFormat<'a> {
        status: u32,
        code: &'a str,
        message: &'a str,
    }

    #[test]
    fn deserialize_error_code() {
        for code in ERROR_CODES {
            let format = ErrorFormat {
                status: 200,
                code: code,
                message: "Irrelevant",
            };

            let error: MaybeTyped =
                serde_json::from_str(&serde_json::to_string(&format).unwrap()).unwrap();
            assert!(error.is_typed(), "{}: {:#?}", code, APIError::from(error));
        }
    }
}
