//! IP structs and implementation.

use std::{collections::HashMap, net::Ipv4Addr};

use bytesize::ByteSize;
use serde::{Deserialize, Serialize};
use time::Date;

use crate::{error::Error, AsyncRobot};

use super::{
    server::ServerId,
    wrapper::{List, Single},
    UnauthenticatedRequest,
};

fn list_ips() -> UnauthenticatedRequest<List<Ip>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/ip")
}

fn get_ip(ip: Ipv4Addr) -> UnauthenticatedRequest<Single<Ip>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/ip/{ip}"))
}

fn enable_traffic_warnings(
    ip: Ipv4Addr,
    traffic_warnings: Option<TrafficWarnings>,
) -> Result<UnauthenticatedRequest<Single<Ip>>, serde_html_form::ser::Error> {
    let request = UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/ip/{ip}"))
        .with_method("POST");

    if let Some(warnings) = traffic_warnings {
        request.with_body(InternalTrafficWarnings::from(warnings))
    } else {
        Ok(request.with_serialized_body("traffic_warnings=true".to_string()))
    }
}

fn disable_traffic_warnings(ip: Ipv4Addr) -> UnauthenticatedRequest<Single<Ip>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/ip/{ip}"))
        .with_method("POST")
        .with_serialized_body("traffic_warnings=false".to_string())
}

fn get_separate_mac(ip: Ipv4Addr) -> UnauthenticatedRequest<Single<InternalMac>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/ip/{ip}/mac"))
}

fn generate_separate_mac(ip: Ipv4Addr) -> UnauthenticatedRequest<Single<InternalMac>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/ip/{ip}/mac"))
        .with_method("PUT")
}

fn delete_separate_mac(ip: Ipv4Addr) -> UnauthenticatedRequest<Single<ExecutedMacRemoval>> {
    UnauthenticatedRequest::from(&format!("https://robot-ws.your-server.de/ip/{ip}/mac"))
        .with_method("DELETE")
}

fn get_ip_cancellation(ip: Ipv4Addr) -> UnauthenticatedRequest<Single<Cancellation>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/ip/{ip}/cancellation"
    ))
}

fn cancel_ip(ip: Ipv4Addr, date: Date) -> UnauthenticatedRequest<Single<Cancelled>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/ip/{ip}/cancellation"
    ))
    .with_method("POST")
    .with_serialized_body(format!("cancellation_date={date}"))
}

fn revoke_ip_cancellation(ip: Ipv4Addr) -> UnauthenticatedRequest<Single<Cancellable>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/ip/{ip}/cancellation"
    ))
    .with_method("DELETE")
}

impl AsyncRobot {
    /// List all single IP addresses, grouped by server they are assigned to.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.list_ips().await.unwrap();
    /// # }
    /// ```
    pub async fn list_ips(&self) -> Result<HashMap<ServerId, Vec<Ip>>, Error> {
        let mut ips: HashMap<ServerId, Vec<Ip>> = HashMap::new();

        for ip in self.go(list_ips()).await?.0 {
            ips.entry(ip.server_number).or_default().push(ip);
        }

        Ok(ips)
    }

    /// Get information about a single IP address.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_ip("123.123.123.123".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn get_ip(&self, ip: Ipv4Addr) -> Result<Ip, Error> {
        Ok(self.go(get_ip(ip)).await?.0)
    }

    /// Enable traffic warnings for the IP address, optionally overriding
    /// the existing traffic limits.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::ip::TrafficWarnings;
    /// # use hrobot::bytesize::ByteSize;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.enable_ip_traffic_warnings(
    ///     "123.123.123.123".parse().unwrap(),
    ///     Some(TrafficWarnings {
    ///         hourly:  ByteSize::mib(200),
    ///         daily:   ByteSize::gib(2),
    ///         monthly: ByteSize::gib(20),
    ///     })
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn enable_ip_traffic_warnings(
        &self,
        ip: Ipv4Addr,
        traffic_warnings: Option<TrafficWarnings>,
    ) -> Result<Ip, Error> {
        Ok(self
            .go(enable_traffic_warnings(ip, traffic_warnings)?)
            .await?
            .0)
    }

    /// Disable traffic warnings for the IP address.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.disable_ip_traffic_warnings("123.123.123.123".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn disable_ip_traffic_warnings(&self, ip: Ipv4Addr) -> Result<Ip, Error> {
        Ok(self.go(disable_traffic_warnings(ip)).await?.0)
    }

    /// Get the separate MAC address for this IP address.
    ///
    /// Note that only non-primary IPv4 addresses can have separate MACs set.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_ip_separate_mac("123.123.123.123".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn get_ip_separate_mac(&self, ip: Ipv4Addr) -> Result<String, Error> {
        Ok(self.go(get_separate_mac(ip)).await?.0.mac)
    }

    /// Generate a separate MAC address for an IP address.
    ///
    /// Note that only non-primary IPv4 addresses can have separate MACs set.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.generate_ip_separate_mac("123.123.123.123".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn generate_ip_separate_mac(&self, ip: Ipv4Addr) -> Result<String, Error> {
        Ok(self.go(generate_separate_mac(ip)).await?.0.mac)
    }

    /// Remove the separate MAC address for an IP address.
    ///
    /// Note that only non-primary IPv4 addresses can have separate MACs set.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.remove_ip_separate_mac("123.123.123.123".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn remove_ip_separate_mac(&self, ip: Ipv4Addr) -> Result<(), Error> {
        self.go(delete_separate_mac(ip)).await.map(|_| ())
    }

    /// Get cancellation status for a single IP address.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_ip_cancellation("123.123.123.123".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn get_ip_cancellation(&self, ip: Ipv4Addr) -> Result<Cancellation, Error> {
        Ok(self.go(get_ip_cancellation(ip)).await?.0)
    }

    /// Cancel an IP address.
    ///
    /// # Example
    /// ```rust,no_run
    /// use time::{Date, Month};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.cancel_ip(
    ///     "123.123.123.123".parse().unwrap(),
    ///     Date::from_calendar_date(2023, Month::July, 17).unwrap()
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn cancel_ip(&self, ip: Ipv4Addr, date: Date) -> Result<Cancelled, Error> {
        Ok(self.go(cancel_ip(ip, date)).await?.0)
    }

    /// Revoke IP address cancellation.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.revoke_ip_cancellation("123.123.123.123".parse().unwrap()).await.unwrap();
    /// # }
    /// ```
    pub async fn revoke_ip_cancellation(&self, ip: Ipv4Addr) -> Result<Cancellable, Error> {
        Ok(self.go(revoke_ip_cancellation(ip)).await?.0)
    }
}

/// Traffic warning configuration.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(try_from = "InternalTrafficWarnings")]
pub struct TrafficWarnings {
    /// Produce a warning if the hourly traffic exceeds this limit.
    #[serde(rename = "traffic_hourly")]
    #[serde(with = "crate::conversion::mib")]
    pub hourly: ByteSize,

    /// Produce a warning if the daily traffic exceeds this limit.
    #[serde(rename = "traffic_daily")]
    #[serde(with = "crate::conversion::mib")]
    pub daily: ByteSize,

    /// Produce a warning if the monthly traffic exceeds this limit.
    #[serde(rename = "traffic_monthly")]
    #[serde(with = "crate::conversion::gib")]
    pub monthly: ByteSize,
}

// This is the default configuration for servers,
// as specified by Hetzner.
impl Default for TrafficWarnings {
    fn default() -> Self {
        TrafficWarnings {
            hourly: ByteSize::mib(200),
            daily: ByteSize::mib(2000),
            monthly: ByteSize::gib(20),
        }
    }
}

// This structure is used to deserialize and convert from for traffic warnings,
// yielding None if the traffic warnings are disabled.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct InternalTrafficWarnings {
    traffic_warnings: bool,
    #[serde(with = "crate::conversion::mib")]
    traffic_hourly: ByteSize,
    #[serde(with = "crate::conversion::mib")]
    traffic_daily: ByteSize,
    #[serde(with = "crate::conversion::gib")]
    traffic_monthly: ByteSize,
}

impl TryFrom<InternalTrafficWarnings> for TrafficWarnings {
    type Error = &'static str;

    fn try_from(value: InternalTrafficWarnings) -> Result<Self, Self::Error> {
        if value.traffic_warnings {
            Ok(TrafficWarnings {
                hourly: value.traffic_hourly,
                daily: value.traffic_daily,
                monthly: value.traffic_monthly,
            })
        } else {
            Err("traffic warnings disabled")
        }
    }
}

impl From<TrafficWarnings> for InternalTrafficWarnings {
    fn from(value: TrafficWarnings) -> Self {
        InternalTrafficWarnings {
            traffic_warnings: true,
            traffic_hourly: value.hourly,
            traffic_daily: value.daily,
            traffic_monthly: value.monthly,
        }
    }
}

/// Describes a network.
#[derive(Debug, Clone, Deserialize)]
pub struct Network {
    /// Gateway for the IP address.
    pub gateway: Ipv4Addr,

    /// Netmask for the IP address.
    pub mask: u8,

    /// Broadcast address for the IP address.
    pub broadcast: Ipv4Addr,
}

/// Describes a single server-attached IPv4 Address.
#[derive(Debug, Clone, Deserialize)]
pub struct Ip {
    /// Address
    pub ip: Ipv4Addr,

    /// Server the ip belongs to
    pub server_number: ServerId,

    /// Status of locking.
    pub locked: bool,

    /// Network subnet this IP address exists in.
    ///
    /// This field is only available when retrieving the address directly ([`AsyncRobot::list_ips()`])
    /// and is not returned when listing IP addresses generally ([`AsyncRobot::get_ip()`])
    #[serde(flatten)]
    pub network: Option<Network>,

    /// Separate MAC address, if any.
    pub separate_mac: Option<String>,

    /// Traffic warnings for this IP address.
    #[serde(flatten)]
    pub traffic_warnings: Option<TrafficWarnings>,
}

#[derive(Deserialize)]
pub(crate) struct InternalMac {
    pub mac: String,
}

/// Deleting the separate MAC address for an IP returns a `mac` object
/// with the MAC address set to null. Since our internal representation
/// of a MAC address is not nullable, we use this struct here for that
/// specific response, and then just void the information.
#[derive(Deserialize)]
pub(crate) struct ExecutedMacRemoval {
    #[serde(rename = "ip")]
    _ip: Ipv4Addr,
}

/// IP address has been cancelled.
#[derive(Debug, Clone, Deserialize)]
pub struct Cancelled {
    /// Date at which the IP address is terminated.
    #[serde(rename = "cancellation_date")]
    pub date: Date,
}

/// IP address has not yet been cancelled.
#[derive(Debug, Clone, Deserialize)]
pub struct Cancellable {
    /// Earliest possible date at which the IP address can be cancelled.
    pub earliest_cancellation_date: Date,
}

/// Describes the cancellation state of the IP address.
///
/// If the address has already been cancelled, this contains a [`Cancelled`]
/// otherwise a [`Cancellable`] structure which describes the earliest date
/// at which the IP address can be cancelled.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Cancellation {
    /// IP address has been cancelled.
    Cancelled(Cancelled),
    /// IP address has not yet been cancelled.
    Cancellable(Cancellable),
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "non-disruptive-tests")]
    mod non_disruptive_tests {
        use tracing::info;
        use tracing_test::traced_test;

        use crate::error::{ApiError, Error};

        #[tokio::test]
        #[traced_test]
        async fn test_list_ips() {
            let _ = dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();
            let ips = robot.list_ips().await.unwrap();

            info!("{ips:#?}");
        }

        #[tokio::test]
        #[traced_test]
        async fn test_get_server_ip_information() {
            let _ = dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let servers = robot.list_servers().await.unwrap();
            info!("{servers:#?}");

            if let Some(ip) = servers.into_iter().find_map(|server| server.ipv4) {
                let ip = robot.get_ip(ip).await.unwrap();
                info!("{ip:#?}");
            }
        }

        #[tokio::test]
        #[traced_test]
        async fn test_get_separate_mac() {
            let _ = dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let servers = robot.list_servers().await.unwrap();
            info!("{servers:#?}");

            if let Some(ip) = servers.into_iter().find_map(|server| server.ipv4) {
                // Server primary IPs do not have configurable MAC addresses
                assert!(matches!(
                    robot.get_ip_separate_mac(ip).await,
                    Err(Error::Api(ApiError::MacNotAvailable { .. })),
                ));
            }
        }

        #[tokio::test]
        #[traced_test]
        async fn test_get_server_ip_cancellation() {
            let _ = dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let servers = robot.list_servers().await.unwrap();
            info!("{servers:#?}");

            if let Some(ip) = servers.into_iter().find_map(|server| server.ipv4) {
                let cancellation = robot.get_ip_cancellation(ip).await.unwrap();
                info!("{cancellation:#?}");
            }
        }
    }

    #[cfg(feature = "disruptive-tests")]
    mod disruptive_tests {
        use serial_test::serial;
        use tracing::info;
        use tracing_test::traced_test;

        use crate::api::ip::TrafficWarnings;

        #[tokio::test]
        #[traced_test]
        #[serial("ip")]
        #[ignore = "unexpected failure can leave the traffic warning in undesired configuration"]
        async fn test_enable_and_disable_traffic_warnings() {
            let _ = dotenvy::dotenv().ok();

            let robot = crate::AsyncRobot::default();

            let servers = robot.list_servers().await.unwrap();
            info!("{servers:#?}");

            if let Some(ip) = servers.into_iter().find_map(|server| server.ipv4) {
                let ip = robot.get_ip(ip).await.unwrap();
                info!("{ip:#?}");

                let original_traffic_warning = ip.traffic_warnings;

                let new_warnings = robot
                    .enable_ip_traffic_warnings(ip.ip, Some(TrafficWarnings::default()))
                    .await
                    .unwrap();

                assert_eq!(
                    new_warnings.traffic_warnings.unwrap(),
                    TrafficWarnings::default()
                );

                let _ = robot.disable_ip_traffic_warnings(ip.ip).await.unwrap();

                // Restore the original traffic warning settings.
                if let Some(warnings) = original_traffic_warning {
                    let _ = robot
                        .enable_ip_traffic_warnings(ip.ip, Some(warnings))
                        .await
                        .unwrap();
                }
            }
        }
    }
}
