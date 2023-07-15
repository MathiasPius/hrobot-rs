use crate::models::urlencode::{UrlEncode, UrlEncodingBuffer};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, net::Ipv4Addr, ops::RangeInclusive};

/// Version of the Internet Protocol supported by the firewall.
#[derive(Default, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IPVersion {
    /// IPv4
    #[default]
    IPv4,
    /// IPv6
    IPv6,
}

impl AsRef<str> for IPVersion {
    fn as_ref(&self) -> &str {
        match self {
            IPVersion::IPv4 => "ipv4",
            IPVersion::IPv6 => "ipv6",
        }
    }
}

impl Display for IPVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum State {
    /// Firewall is active.
    #[serde(rename = "active")]
    Active,

    /// Firewall is currently processing a request.
    #[serde(rename = "in process")]
    InProcess,

    /// Firewall is disabled.
    #[serde(rename = "disabled")]
    Disabled,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Active => "active",
                State::InProcess => "in process",
                State::Disabled => "disabled",
            }
        )
    }
}

/// Switch port of the server.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SwitchPort {
    #[default]
    /// Primary port.
    Main,
    /// Port used for KVM access.
    Kvm,
}

/// Protocol types which can be used by rules.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    /// Transmission Control Protocol.
    TCP,

    /// User Datagram Protocol.
    UDP,

    /// Generic Routing Encapsulation.
    GRE,

    /// Internet Control Message Protocol.
    ICMP,

    /// IP-in-IP tunneling.
    IPIP,

    /// IPSec Authentication Header.
    AH,

    /// IPSec Encapsulating Security Payload.
    ESP,
}

impl AsRef<str> for Protocol {
    fn as_ref(&self) -> &str {
        match self {
            Protocol::TCP => "tcp",
            Protocol::UDP => "udp",
            Protocol::GRE => "gre",
            Protocol::ICMP => "icmp",
            Protocol::IPIP => "ipip",
            Protocol::AH => "ah",
            Protocol::ESP => "esp",
        }
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

/// Course of action to take when a rule matches.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    /// Explicitly accept the packet.
    #[default]
    Accept,

    /// Explicitly discard (or "drop") the packet.
    Discard,
}

impl AsRef<str> for Action {
    fn as_ref(&self) -> &str {
        match self {
            Action::Accept => "accept",
            Action::Discard => "discard",
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

/// Describes a Firewall template.
///
/// This is only a descriptor for a template, it does not contain
/// any firewall rules.
#[derive(Debug, Clone, Deserialize)]
pub struct FirewallTemplateReference {
    /// Unique template ID. Can be used to fetch the entire rule
    /// list using [`AsyncRobot::get_firewall_template()`]
    pub id: u32,

    /// Human-readable name for the template.
    pub name: String,

    /// Whether to filter IPv6 traffic.
    pub filter_ipv6: bool,

    /// Whether to whitelist Hetzner's services,
    /// granting them access through the firewall.
    #[serde(rename = "whitelist_hos")]
    pub whitelist_hetzner_services: bool,

    /// Indicates if this template is set as default.
    pub is_default: bool,
}

/// Describes an entire firewall template.
#[derive(Debug, Clone, Deserialize)]
pub struct FirewallTemplate {
    /// Unique firewall template id
    pub id: u32,

    /// Human-readable name for the template.
    pub name: String,

    /// Whether to filter IPv6 traffic.
    pub filter_ipv6: bool,

    /// Whether to whitelist Hetzner's services,
    /// granting them access through the firewall.
    #[serde(rename = "whitelist_hos")]
    pub whitelist_hetzner_services: bool,

    /// Indicates whether this template shows up as the
    /// default in the Robot webpanel.
    pub is_default: bool,

    /// Firewall rules defined for this Firewall.
    pub rules: Rules,
}

#[derive(Debug, Clone)]
pub struct FirewallTemplateConfiguration {
    /// Human-readable name for the template.
    pub name: String,

    /// Whether to filter IPv6 traffic.
    pub filter_ipv6: bool,

    /// Whether to whitelist Hetzner's services,
    /// granting them access through the firewall.
    pub whitelist_hetzner_services: bool,

    /// Indicates whether this template shows up as the
    /// default in the Robot webpanel.
    pub is_default: bool,

    /// Firewall rules defined for this Firewall.
    pub rules: Rules,
}

/// Describes an entire Firewall for a server.
///
/// This is returned by Hetzner when getting or updating the firewall of a server.
/// For configuring the firewall, instead use the [`FirewallConfiguration`] struct,
/// which can also be extracted using [`Firewall::configuration()`]
#[derive(Debug, Clone, Deserialize)]
pub struct Firewall {
    /// Primary IPv4 address of the server which this firewall applies to.
    #[serde(rename = "server_ip")]
    pub ipv4: Ipv4Addr,

    /// Unique server ID.
    #[serde(rename = "server_number")]
    pub id: u32,

    /// Status of the server's firewall.
    pub status: State,

    /// Whether to filter IPv6 traffic.
    pub filter_ipv6: bool,

    /// Whether to whitelist Hetzner's services,
    /// granting them access through the firewall.
    #[serde(rename = "whitelist_hos")]
    pub whitelist_hetzner_services: bool,

    /// Switch of the server to which this firewall applies.
    pub port: SwitchPort,

    /// Firewall rules defined for this Firewall.
    pub rules: Rules,
}

impl Firewall {
    /// Extract the firewall configuration from this firewall description.
    pub fn configuration(&self) -> FirewallConfiguration {
        self.into()
    }
}

/// Firewall configuration to apply to a server.
#[derive(Debug)]
pub struct FirewallConfiguration {
    /// Status of the server's firewall.
    pub status: State,

    /// Whether to filter IPv6 traffic.
    pub filter_ipv6: bool,

    /// Whether to whitelist Hetzner's services,
    /// granting them access through the firewall.
    pub whitelist_hetzner_services: bool,

    /// Firewall rules defined for this Firewall.
    pub rules: Rules,
}

impl From<&Firewall> for FirewallConfiguration {
    fn from(value: &Firewall) -> Self {
        FirewallConfiguration {
            status: value.status.clone(),
            filter_ipv6: value.filter_ipv6,
            whitelist_hetzner_services: value.whitelist_hetzner_services,
            rules: value.rules.clone(),
        }
    }
}

/// Encapsulates all ingoing and outgoing rules for a Firewall.
#[derive(Debug, Clone, Deserialize)]
pub struct Rules {
    /// Rules applied to ingress traffic (traffic to the server).
    #[serde(rename = "input", default, skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<Rule>,

    /// Rules applied to egress traffic (traffic leaving the server).
    #[serde(rename = "output", default, skip_serializing_if = "Vec::is_empty")]
    pub egress: Vec<Rule>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortRange(RangeInclusive<u16>);

impl PortRange {
    /// Construct a port range covering only a single port.
    ///
    /// Equivalent to PortRange::range(port, port);
    ///
    /// # Example
    /// ```rust
    /// # use hrobot::models::PortRange;
    /// // Cover only HTTPS.
    /// let range = PortRange::port(443);
    /// ```
    pub fn port(port: u16) -> Self {
        PortRange(RangeInclusive::new(port, port))
    }

    /// Construct a port range given the start and end port (inclusive)
    ///
    /// # Example
    /// ```rust
    /// # use hrobot::models::PortRange;
    /// // Covers all ephemeral ports
    /// let range = PortRange::range(32768, 60999);
    /// ```
    pub fn range(start: u16, end: u16) -> Self {
        PortRange(RangeInclusive::new(start, end))
    }

    /// Get the first port in the range.
    pub fn start(&self) -> u16 {
        *self.0.start()
    }

    /// Get the last port in the range.
    pub fn end(&self) -> u16 {
        *self.0.end()
    }
}

impl Display for PortRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.start())?;
        if self.end() != self.start() {
            write!(f, "-{}", self.end())?;
        }

        Ok(())
    }
}

impl From<RangeInclusive<u16>> for PortRange {
    fn from(value: RangeInclusive<u16>) -> Self {
        PortRange(value)
    }
}

impl From<&RangeInclusive<u16>> for PortRange {
    fn from(value: &RangeInclusive<u16>) -> Self {
        PortRange(value.clone())
    }
}

impl From<PortRange> for RangeInclusive<u16> {
    fn from(value: PortRange) -> Self {
        value.0
    }
}

impl From<&PortRange> for RangeInclusive<u16> {
    fn from(value: &PortRange) -> Self {
        value.0.clone()
    }
}

impl IntoIterator for PortRange {
    type Item = u16;

    type IntoIter = <RangeInclusive<u16> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'de> Deserialize<'de> for PortRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let value: &str = Deserialize::deserialize(deserializer)?;

        if let Some((start, end)) = value.split_once('-') {
            let start = u16::from_str_radix(start, 10).map_err(D::Error::custom)?;
            let end = u16::from_str_radix(end, 10).map_err(D::Error::custom)?;

            Ok(PortRange(RangeInclusive::new(start, end)))
        } else {
            let port = u16::from_str_radix(value, 10).map_err(D::Error::custom)?;

            Ok(PortRange(RangeInclusive::new(port, port)))
        }
    }
}

/// Describes a single Firewall rule.
#[derive(Default, Clone, PartialEq, Eq, Debug, Deserialize)]
pub struct Rule {
    /// IP version which this rule applies to. None implies both.
    pub ip_version: Option<IPVersion>,

    /// Human-readable name for the rule.
    pub name: String,

    /// Destination IP address.
    pub dst_ip: Option<String>,

    /// Source IP address.
    pub src_ip: Option<String>,

    /// Destination Port.
    pub dst_port: Option<PortRange>,

    /// Source Port.
    pub src_port: Option<PortRange>,

    /// Protocol
    pub protocol: Option<Protocol>,

    /// TCP Flags.
    pub tcp_flags: Option<String>,

    /// Action to take if rule matches.
    pub action: Action,
}

impl UrlEncode for Rule {
    fn encode_into(&self, mut f: UrlEncodingBuffer<'_>) {
        f.set("[name]", &self.name);

        if let Some(ip_version) = self.ip_version.as_ref() {
            f.set("[ip_version]", ip_version)
        }

        if let Some(dst_ip) = self.dst_ip.as_ref() {
            f.set("[dst_ip]", dst_ip)
        }

        if let Some(src_ip) = self.src_ip.as_ref() {
            f.set("[src_ip]", src_ip)
        }

        if let Some(dst_port) = self.dst_port.as_ref() {
            f.set("[dst_port]", dst_port)
        }

        if let Some(src_port) = self.src_port.as_ref() {
            f.set("[src_port]", src_port)
        }

        if let Some(protocol) = self.protocol.as_ref() {
            f.set("[protocol]", protocol)
        }

        if let Some(tcp_flags) = self.tcp_flags.as_ref() {
            f.set("[tcp_flags]", tcp_flags)
        }

        f.set("[action]", self.action);
    }
}

impl UrlEncode for Rules {
    fn encode_into(&self, mut f: UrlEncodingBuffer<'_>) {
        {
            let mut ingress = f.append("[input]");
            for (index, rule) in self.ingress.iter().enumerate() {
                rule.encode_into(ingress.append(&format!("[{index}]")));
            }

            let mut egress = f.append("[output]");
            for (index, rule) in self.ingress.iter().enumerate() {
                rule.encode_into(egress.append(&format!("[{index}]")));
            }
        }
    }
}

impl UrlEncode for FirewallConfiguration {
    fn encode_into(&self, mut f: UrlEncodingBuffer<'_>) {
        f.set("status", self.status);
        f.set("filter_ipv6", self.filter_ipv6);
        f.set("whitelist_hos", self.whitelist_hetzner_services);
        self.rules.encode_into(f.append("rules"));
    }
}

impl UrlEncode for FirewallTemplateConfiguration {
    fn encode_into(&self, mut f: UrlEncodingBuffer<'_>) {
        f.set("name", &self.name);
        f.set("filter_ipv6", self.filter_ipv6);
        f.set("whitelist_hos", self.whitelist_hetzner_services);
        f.set("is_default", self.is_default);
        self.rules.encode_into(f.append("rules"));
    }
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    use crate::models::{
        Action, FirewallConfiguration, IPVersion, PortRange, Protocol, Rule, Rules, State,
    };

    use super::UrlEncode;

    #[test]
    #[traced_test]
    fn serialize_firewall_config() {
        info!(
            "{}",
            &FirewallConfiguration {
                status: State::Active,
                filter_ipv6: false,
                whitelist_hetzner_services: true,
                rules: Rules {
                    ingress: vec![Rule {
                        ip_version: Some(IPVersion::IPv4),
                        name: "Some rule".to_owned(),
                        dst_ip: Some("127.0.0.0/8".to_string()),
                        src_ip: Some("0.0.0.0/0".to_string()),
                        dst_port: Some(PortRange::range(27015, 27016)),
                        src_port: None,
                        protocol: Some(Protocol::TCP),
                        tcp_flags: None,
                        action: Action::Accept,
                    }],
                    egress: vec![Rule {
                        ip_version: None,
                        name: "Allow all".to_owned(),
                        dst_ip: None,
                        src_ip: None,
                        dst_port: None,
                        src_port: None,
                        protocol: None,
                        tcp_flags: None,
                        action: Action::Accept,
                    }],
                },
            }
            .encode()
        );
    }
}
