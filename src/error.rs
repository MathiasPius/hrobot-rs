use std::fmt::Display;

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize, Error)]
#[serde(tag = "code", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum APIError {
    #[error("resource unavailable")]
    Unavailable,
    #[error("not found: {message}")]
    NotFound { message: String },
    #[error("server not found: {message}")]
    ServerNotFound { message: String },
    #[error("ip address not found: {message}")]
    IpNotFound { message: String },
    #[error("subnet not found: {message}")]
    SubnetNotFound { message: String },
    #[error("mac address not found: {message}")]
    MacNotFound { message: String },
    #[error("mac address not available: {message}")]
    MacNotAvailable { message: String },
    #[error("mac address already set: {message}")]
    MacAlreadySet { message: String },
    #[error("mac address failure: {message}")]
    MacFailed { message: String },
    #[error("wak-on-lan not available: {message}")]
    WolNotAvailable { message: String },
    #[error("wake-on-lan failed: {message}")]
    WolFailed { message: String },
    #[error("outdated windows version: {message}")]
    WindowsOutdatedVersion { message: String },
    #[error("windows addon missing: {message}")]
    WindowsMissingAddon { message: String },
    #[error("plesk addon missing: {message}")]
    PleskMissingAddon { message: String },
    #[error("cpanel addon missing: {message}")]
    CpanelMissingAddon { message: String },
    #[error("rate limit exceeded: {message} (max req: {max_request}, interval: {interval}")]
    RateLimitExceeded {
        message: String,
        max_request: u32,
        interval: u32,
    },
    #[error("reset not available: {message}")]
    ResetNotAvailable { message: String },
    #[error("storage box not found: {message}")]
    StorageboxNotFound { message: String },
    #[error("storage box sub-account not found: {message}")]
    StorageboxSubaccountNotFound { message: String },
    #[error("stoage box sub-account limit exceeded: {message}")]
    StorageboxSubaccountLimitExceeded { message: String },
    #[error("snapshot not found: {message}")]
    SnapshotNotFound { message: String },
    #[error("snapshot limit exceeded: {message}")]
    SnapshotLimitExceeded { message: String },
    #[error("firewall port not found: {message}")]
    FirewallPortNotFound { message: String },
    #[error("firewall not available: {message}")]
    FirewallNotAvailable { message: String },
    #[error("firewall template not found: {message}")]
    FirewallTemplateNotFound { message: String },
    #[error("firewall is already processing a request: {message}")]
    FirewallInProcess { message: String },
    #[error("vSwitch limit reached: {message}")]
    VswitchLimitReached { message: String },
    #[error("vswitch not available: {message}")]
    VswitchNotAvailable { message: String },
    #[error("vSwitch server limit reached: {message}")]
    VswitchServerLimitReached { message: String },
    #[error("vSwitch-per-server limit reached: {message}")]
    VswitchPerServerLimitReached { message: String },
    #[error("vSwitch is already processing a request: {message}")]
    VswitchInProcess { message: String },
    #[error("vSwitch VLAN-ID must be unique: {message}")]
    VswitchVlanNotUnique { message: String },
    #[error("manual reset is active: {message}")]
    ResetManualActive { message: String },
    #[error("key update failed: {message}")]
    KeyUpdateFailed { message: String },
    #[error("key creation failed: {message}")]
    KeyCreateFailed { message: String },
    #[error("key deletion failed: {message}")]
    KeyDeleteFailed { message: String },
    #[error("key already exists: {message}")]
    KeyAlreadyExists { message: String },
    #[error("rnds entry not found: {message}")]
    RdnsNotFound { message: String },
    #[error("rdns creation failed: {message}")]
    RdnsCreateFailed { message: String },
    #[error("rdns update failed: {message}")]
    RdnsUpdateFailed { message: String },
    #[error("rnds deletion failed: {message}")]
    RdnsDeleteFailed { message: String },
    #[error("rnds entry already exists: {message}")]
    RdnsAlreadyExists { message: String },
    #[error("reset failed: {message}")]
    ResetFailed { message: String },
    #[error("invalid input: {message}")]
    InvalidInput {
        message: String,
        #[serde(default)]
        missing: Vec<String>,
        #[serde(default)]
        invalid: Vec<String>,
    },
    #[error("conflict: {message}")]
    Conflict { message: String },
    #[error("server cancellation reserve location must be false: {message}")]
    ServerCancellationReserveLocationFalseOnly { message: String },
    #[error("traffic warning update failed: {message}")]
    TrafficWarningUpdateFailed { message: String },
    #[error("boot not available: {message}")]
    BootNotAvailable { message: String },
    #[error("internal error: {message}")]
    InternalError { message: String },
    #[error("failover already routed: {message}")]
    FailoverAlreadyRouted { message: String },
    #[error("failover failed: {message}")]
    FailoverFailed { message: String },
    #[error("failover locked: {message}")]
    FailoverLocked { message: String },
    #[error("failover not complete: {message}")]
    FailoverNotComplete { message: String },
    #[error("new failover server not found: {message}")]
    FailoverNewServerNotFound { message: String },
    #[error("server reversal not possible: {message}")]
    ServerReversalNotPossible { message: String },
    #[error("boot activation failed: {message}")]
    BootActivationFailed { message: String },
    #[error("boot deactivation failed: {message}")]
    BootDeactivationFailed { message: String },
    #[error("boot already enabled: {message}")]
    BootAlreadyEnabled { message: String },
    #[error("boot locked: {message}")]
    BootBlocked { message: String },
    #[serde(skip_deserializing)]
    #[error("unknown error {0}")]
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

impl Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unclassified error: {status} {code}: {message}",
            status = self.status,
            code = self.code,
            message = self.message
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum MaybeTyped {
    Typed(APIError),
    Untyped(GenericError),
}

#[derive(Debug, Deserialize)]
pub(crate) struct MaybeTypedResponse {
    pub error: MaybeTyped,
}

impl From<MaybeTypedResponse> for MaybeTyped {
    fn from(m: MaybeTypedResponse) -> Self {
        m.error
    }
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
pub(crate) enum APIResult<T> {
    Ok(T),
    Error(MaybeTypedResponse),
}

impl<T> From<APIResult<T>> for Result<T, Error> {
    fn from(result: APIResult<T>) -> Self {
        match result {
            APIResult::Ok(inner) => Ok(inner),
            APIResult::Error(e) => Err(Error::API(e.error.into())),
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("transport error: {0}")]
    Transport(#[from] Box<dyn std::error::Error>),
    #[error("json decode error: {0}")]
    Decode(#[from] serde_json::Error),
    #[error("api error: {0}")]
    API(#[from] APIError),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Transport(Box::new(e))
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
