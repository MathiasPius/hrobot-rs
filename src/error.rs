use std::fmt::Display;

use serde::{Deserialize, Deserializer};
use thiserror::Error;

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// Error returned by the Hetzner Robot API.
#[derive(Debug, Deserialize, Error)]
#[serde(tag = "code", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiError {
    /// Resource Unavailable.
    #[error("resource unavailable")]
    Unavailable,

    /// Resource not found.
    #[error("not found: {message}")]
    NotFound { message: String },

    /// Server not found.
    #[error("server not found: {message}")]
    ServerNotFound { message: String },

    /// IP address not found.
    #[error("ip address not found: {message}")]
    IpNotFound { message: String },

    /// Subnet not found.
    #[error("subnet not found: {message}")]
    SubnetNotFound { message: String },

    /// MAC address not found.
    #[error("mac address not found: {message}")]
    MacNotFound { message: String },

    /// MAC address not available.
    #[error("mac address not available: {message}")]
    MacNotAvailable { message: String },

    /// MAC address already set.
    #[error("mac address already set: {message}")]
    MacAlreadySet { message: String },

    /// MAC address failure.
    #[error("mac address failure: {message}")]
    MacFailed { message: String },

    /// Wake-on-LAN not available.
    #[error("wak-on-lan not available: {message}")]
    WolNotAvailable { message: String },

    // Wake-on-LAN failed.
    #[error("wake-on-lan failed: {message}")]
    WolFailed { message: String },

    /// Outdated Windows version.
    #[error("outdated windows version: {message}")]
    WindowsOutdatedVersion { message: String },

    /// Missing Windows addon.
    #[error("windows addon missing: {message}")]
    WindowsMissingAddon { message: String },

    /// Missing Plesk addon.
    #[error("plesk addon missing: {message}")]
    PleskMissingAddon { message: String },

    /// Missing CPanel addon.
    #[error("cpanel addon missing: {message}")]
    CpanelMissingAddon { message: String },

    /// API Rate limit exceeded.
    #[error("rate limit exceeded: {message} (max req: {max_request}, interval: {interval}")]
    RateLimitExceeded {
        message: String,
        max_request: u32,
        interval: u32,
    },

    /// Reset not available.
    #[error("reset not available: {message}")]
    ResetNotAvailable { message: String },

    /// Storage Box not found.
    #[error("storage box not found: {message}")]
    StorageboxNotFound { message: String },

    /// Storage Box sub-account not found.
    #[error("storage box sub-account not found: {message}")]
    StorageboxSubaccountNotFound { message: String },

    /// Storage Box sub-account limit exceeded.
    #[error("stoage box sub-account limit exceeded: {message}")]
    StorageboxSubaccountLimitExceeded { message: String },

    /// Snapshot not found.
    #[error("snapshot not found: {message}")]
    SnapshotNotFound { message: String },

    /// Snapshot limit exceeded.
    #[error("snapshot limit exceeded: {message}")]
    SnapshotLimitExceeded { message: String },

    /// Firewall port not found.
    #[error("firewall port not found: {message}")]
    FirewallPortNotFound { message: String },

    /// Firewall not available.
    #[error("firewall not available: {message}")]
    FirewallNotAvailable { message: String },

    /// Firewall template not found.
    #[error("firewall template not found: {message}")]
    FirewallTemplateNotFound { message: String },

    /// Firewall is already processing a request.
    #[error("firewall is already processing a request: {message}")]
    FirewallInProcess { message: String },

    /// vSwitch limit reached.
    #[error("vSwitch limit reached: {message}")]
    VswitchLimitReached { message: String },

    /// vSwitch not available.
    #[error("vswitch not available: {message}")]
    VswitchNotAvailable { message: String },

    /// vSwitch server limit reached.
    #[error("vSwitch server limit reached: {message}")]
    VswitchServerLimitReached { message: String },

    /// vSwitch-per-server limit reached.
    #[error("vSwitch-per-server limit reached: {message}")]
    VswitchPerServerLimitReached { message: String },

    /// vSwitch is already processing a request.
    #[error("vSwitch is already processing a request: {message}")]
    VswitchInProcess { message: String },

    /// vSwitch VLAN-ID is not unique.
    #[error("vSwitch VLAN-ID must be unique: {message}")]
    VswitchVlanNotUnique { message: String },

    /// Manual reset is active.
    #[error("manual reset is active: {message}")]
    ResetManualActive { message: String },

    /// Key update failed.
    #[error("key update failed: {message}")]
    KeyUpdateFailed { message: String },

    /// Key creation failed.
    #[error("key creation failed: {message}")]
    KeyCreateFailed { message: String },

    /// Key deletion failed.
    #[error("key deletion failed: {message}")]
    KeyDeleteFailed { message: String },

    /// Key already exists.
    #[error("key already exists: {message}")]
    KeyAlreadyExists { message: String },

    /// Reverse DNS entry not found.
    #[error("rnds entry not found: {message}")]
    RdnsNotFound { message: String },

    /// Reverse DNS entry creation failed.
    #[error("rdns creation failed: {message}")]
    RdnsCreateFailed { message: String },

    /// Reverse DNS update failed.
    #[error("rdns update failed: {message}")]
    RdnsUpdateFailed { message: String },

    /// Reverse DNS entry deletion failed.
    #[error("rnds deletion failed: {message}")]
    RdnsDeleteFailed { message: String },

    /// Reverse DNS entry already exists.
    #[error("rnds entry already exists: {message}")]
    RdnsAlreadyExists { message: String },

    /// Reset failed.
    #[error("reset failed: {message}")]
    ResetFailed { message: String },

    /// Invalid input.
    #[error("invalid input: {message}")]
    InvalidInput {
        message: String,
        #[serde(default, deserialize_with = "deserialize_null_default")]
        missing: Vec<String>,
        #[serde(default, deserialize_with = "deserialize_null_default")]
        invalid: Vec<String>,
    },
    /// Conflict.
    #[error("conflict: {message}")]
    Conflict { message: String },

    /// Server cancellation "reserve location" must be false.
    #[error("server cancellation reserve location must be false: {message}")]
    ServerCancellationReserveLocationFalseOnly { message: String },

    /// Traffic warning update failed.
    #[error("traffic warning update failed: {message}")]
    TrafficWarningUpdateFailed { message: String },

    /// Boot is not available.
    #[error("boot not available: {message}")]
    BootNotAvailable { message: String },

    /// Internal Error.
    #[error("internal error: {message}")]
    InternalError { message: String },

    /// Failover is already routed.
    #[error("failover already routed: {message}")]
    FailoverAlreadyRouted { message: String },

    /// Failover failed.
    #[error("failover failed: {message}")]
    FailoverFailed { message: String },

    /// Failover is locked.
    #[error("failover locked: {message}")]
    FailoverLocked { message: String },

    /// Failover not complete.
    #[error("failover not complete: {message}")]
    FailoverNotComplete { message: String },

    /// New failover server not found.
    #[error("new failover server not found: {message}")]
    FailoverNewServerNotFound { message: String },

    /// Withdrawal of server order not possible.
    #[error("withdrawal of server order not possible: {message}")]
    ServerReversalNotPossible { message: String },

    /// Boot activation failed.
    #[error("boot activation failed: {message}")]
    BootActivationFailed { message: String },

    /// Boot deactivation failed.
    #[error("boot deactivation failed: {message}")]
    BootDeactivationFailed { message: String },

    /// Boot already enabled.
    #[error("boot already enabled: {message}")]
    BootAlreadyEnabled { message: String },

    /// Boot blocked.
    #[error("boot locked: {message}")]
    BootBlocked { message: String },

    /// Unknown/generic error.
    #[serde(skip_deserializing)]
    #[error("unknown error {0}")]
    Generic(GenericError),
}

/// Provided input parameters were either incomplete or invalid.
#[derive(Debug, Deserialize)]
pub struct InvalidInputError {
    /// Missing input fields.
    #[serde(default)]
    pub missing: Vec<String>,

    /// Invalid input fields.
    #[serde(default)]
    pub invalid: Vec<String>,
}

/// Hetzner Robot API rate-limit has been exceeded.
#[derive(Debug, Deserialize)]
pub struct RateLimitError {
    /// Time interval in which the [`max_request`](RateLimitError::max_request)
    /// limit applies.
    pub interval: u32,

    /// Maximum number of requests allowed within a given [`interval`](RateLimitError::interval).
    pub max_request: u32,
}

/// Catches generic error cases not explicitly defined in [`ApiError`]
#[derive(Debug, Deserialize)]
pub struct GenericError {
    /// HTTP Status Code, e.g. `404`.
    pub status: u32,

    /// Short error code, e.g. `"BOOT_NOT_AVAILABLE"`
    pub code: String,

    /// Human-readable explanation of the error.
    pub message: String,

    /// Invalid input description.
    ///
    /// Only available if [`code`](GenericError::code)
    /// is `"INVALID_INPUT"`
    #[serde(flatten)]
    pub invalid_input: Option<InvalidInputError>,
    #[serde(flatten)]

    /// Rate limit error description.
    ///
    /// Only available if [`code`](GenericError::code)
    /// is `"RATE_LIMIT_EXCEEDED"`
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
    Typed(ApiError),
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

impl From<MaybeTyped> for ApiError {
    fn from(maybe: MaybeTyped) -> Self {
        match maybe {
            MaybeTyped::Typed(t) => t,
            MaybeTyped::Untyped(t) => ApiError::Generic(t),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ApiResult<T> {
    Ok(T),
    Error(MaybeTypedResponse),
}

impl<T> From<ApiResult<T>> for Result<T, Error> {
    fn from(result: ApiResult<T>) -> Self {
        match result {
            ApiResult::Ok(inner) => Ok(inner),
            ApiResult::Error(e) => Err(Error::Api(e.error.into())),
        }
    }
}

/// Error which can originate at any stage of the API request.
#[derive(Debug, Error)]
pub enum Error {
    /// Covers any errors produced by the Client implementations.
    #[error("transport error: {0}")]
    Transport(#[from] Box<dyn std::error::Error>),
    /// Failure when deserializing json body response from the API.
    #[error("json decode error: {0}")]
    Deserialization(#[from] serde_json::Error),
    /// Failure while attempting to encode the specified input
    /// parameters as `application/x-www-form-urlencoded`
    #[error("html form encoding error: {0}")]
    Serialization(#[from] serde_html_form::ser::Error),
    /// Error returned by the Hetzner Robot API.
    #[error("api error: {0}")]
    Api(#[from] ApiError),
}

impl Error {
    /// Construct an [`Error::Transport`] from the given error.
    ///
    /// Utility function for use with [`Result::map_err()`] specifically.
    pub fn transport(error: impl std::error::Error + 'static) -> Error {
        Error::Transport(Box::new(error))
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
    fn test_deserialize_error_code() {
        for code in ERROR_CODES {
            let format = ErrorFormat {
                status: 200,
                code,
                message: "Irrelevant",
            };

            let error: MaybeTyped =
                serde_json::from_str(&serde_json::to_string(&format).unwrap()).unwrap();
            assert!(error.is_typed(), "{}: {:#?}", code, ApiError::from(error));
        }
    }

    #[test]
    fn test_deserialize_invalid_input() {
        let error = r#"
            {
                "error": {
                    "status":400,
                    "code":"INVALID_INPUT",
                    "message":"invalid input",
                    "missing":null,
                    "invalid":[
                        "rules"
                    ]
                }
            }
        "#;

        let err: MaybeTypedResponse = serde_json::from_str(error).unwrap();

        assert!(matches!(
            err,
            MaybeTypedResponse {
                error: MaybeTyped::Typed(ApiError::InvalidInput { .. })
            }
        ));
    }
}
