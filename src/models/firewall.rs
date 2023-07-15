use crate::models::urlencode::{UrlEncode, UrlEncodingBuffer};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, net::Ipv4Addr, ops::RangeInclusive};

pub use ipnet::Ipv4Net;

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
pub(crate) enum InternalProtocol {
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

impl AsRef<str> for InternalProtocol {
    fn as_ref(&self) -> &str {
        match self {
            InternalProtocol::TCP => "tcp",
            InternalProtocol::UDP => "udp",
            InternalProtocol::GRE => "gre",
            InternalProtocol::ICMP => "icmp",
            InternalProtocol::IPIP => "ipip",
            InternalProtocol::AH => "ah",
            InternalProtocol::ESP => "esp",
        }
    }
}

impl Display for InternalProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

/// Protocol types which can be used by rules.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Protocol {
    /// Transmission Control Protocol.
    TCP { flags: Option<String> },

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

impl From<&Protocol> for InternalProtocol {
    fn from(value: &Protocol) -> Self {
        match value {
            Protocol::TCP { .. } => InternalProtocol::TCP,
            Protocol::UDP => InternalProtocol::UDP,
            Protocol::GRE => InternalProtocol::GRE,
            Protocol::ICMP => InternalProtocol::ICMP,
            Protocol::IPIP => InternalProtocol::IPIP,
            Protocol::AH => InternalProtocol::AH,
            Protocol::ESP => InternalProtocol::ESP,
        }
    }
}

impl Protocol {
    pub fn tcp_with_flags(flags: &str) -> Self {
        Protocol::TCP {
            flags: Some(flags.to_string()),
        }
    }

    fn flags(&self) -> Option<String> {
        match self {
            Protocol::TCP { flags } => flags.clone(),
            _ => None,
        }
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
    /// list using [`AsyncRobot::get_firewall_template()`](crate::AsyncRobot::get_firewall_template)
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
#[derive(Debug, Clone)]
pub struct FirewallTemplate {
    /// Unique firewall template id
    pub id: u32,

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

/// Describes an entire firewall template.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct InternalFirewallTemplate {
    pub id: u32,
    pub name: String,
    pub filter_ipv6: bool,
    #[serde(rename = "whitelist_hos")]
    pub whitelist_hetzner_services: bool,
    pub is_default: bool,
    pub rules: InternalRules,
}

impl From<InternalFirewallTemplate> for FirewallTemplate {
    fn from(value: InternalFirewallTemplate) -> Self {
        FirewallTemplate {
            id: value.id,
            name: value.name,
            filter_ipv6: value.filter_ipv6,
            whitelist_hetzner_services: value.whitelist_hetzner_services,
            is_default: value.is_default,
            rules: value.rules.into(),
        }
    }
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

#[derive(Debug, Clone)]
pub(crate) struct InternalFirewallTemplateConfiguration {
    pub name: String,
    pub filter_ipv6: bool,
    pub whitelist_hetzner_services: bool,
    pub is_default: bool,
    pub rules: InternalRules,
}

impl From<FirewallTemplateConfiguration> for InternalFirewallTemplateConfiguration {
    fn from(value: FirewallTemplateConfiguration) -> Self {
        InternalFirewallTemplateConfiguration {
            name: value.name,
            filter_ipv6: value.filter_ipv6,
            whitelist_hetzner_services: value.whitelist_hetzner_services,
            is_default: value.is_default,
            rules: (&value.rules).into(),
        }
    }
}

/// Describes an entire Firewall for a server.
///
/// This is returned by Hetzner when getting or updating the firewall of a server.
/// For configuring the firewall, instead use the [`FirewallConfiguration`] struct,
/// which can also be extracted using [`Firewall::configuration()`]
#[derive(Debug, Clone)]
pub struct Firewall {
    /// Primary IPv4 address of the server which this firewall applies to.
    pub ipv4: Ipv4Addr,

    /// Unique server ID.
    pub id: u32,

    /// Status of the server's firewall.
    pub status: State,

    /// Whether to filter IPv6 traffic.
    pub filter_ipv6: bool,

    /// Whether to whitelist Hetzner's services,
    /// granting them access through the firewall.
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

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct InternalFirewall {
    #[serde(rename = "server_ip")]
    pub ipv4: Ipv4Addr,
    #[serde(rename = "server_number")]
    pub id: u32,
    pub status: State,
    pub filter_ipv6: bool,
    #[serde(rename = "whitelist_hos")]
    pub whitelist_hetzner_services: bool,
    pub port: SwitchPort,
    pub rules: InternalRules,
}

impl From<InternalFirewall> for Firewall {
    fn from(value: InternalFirewall) -> Self {
        Firewall {
            ipv4: value.ipv4,
            id: value.id,
            status: value.status,
            filter_ipv6: value.filter_ipv6,
            whitelist_hetzner_services: value.whitelist_hetzner_services,
            port: value.port,
            rules: value.rules.into(),
        }
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

pub(crate) struct InternalFirewallConfiguration {
    pub status: State,
    pub filter_ipv6: bool,
    pub whitelist_hetzner_services: bool,
    pub rules: InternalRules,
}

impl From<&FirewallConfiguration> for InternalFirewallConfiguration {
    fn from(value: &FirewallConfiguration) -> Self {
        InternalFirewallConfiguration {
            status: value.status,
            filter_ipv6: value.filter_ipv6,
            whitelist_hetzner_services: value.whitelist_hetzner_services,
            rules: (&value.rules).into(),
        }
    }
}

impl From<&Firewall> for FirewallConfiguration {
    fn from(value: &Firewall) -> Self {
        FirewallConfiguration {
            status: value.status,
            filter_ipv6: value.filter_ipv6,
            whitelist_hetzner_services: value.whitelist_hetzner_services,
            rules: value.rules.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct InternalRules {
    #[serde(rename = "input", default, skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<InternalRule>,

    #[serde(rename = "output", default, skip_serializing_if = "Vec::is_empty")]
    pub egress: Vec<InternalRule>,
}

/// Encapsulates all ingoing and outgoing rules for a Firewall.
#[derive(Debug, Clone)]
pub struct Rules {
    /// Rules applied to ingress traffic (traffic to the server).
    pub ingress: Vec<Rule>,

    /// Rules applied to egress traffic (traffic leaving the server).
    pub egress: Vec<Rule>,
}

impl From<&Rules> for InternalRules {
    fn from(value: &Rules) -> Self {
        InternalRules {
            ingress: value
                .ingress
                .iter()
                .map(Into::<InternalRule>::into)
                .collect(),
            egress: value
                .egress
                .iter()
                .map(Into::<InternalRule>::into)
                .collect(),
        }
    }
}

impl From<InternalRules> for Rules {
    fn from(value: InternalRules) -> Self {
        Rules {
            ingress: value.ingress.into_iter().map(Into::<Rule>::into).collect(),
            egress: value.egress.into_iter().map(Into::<Rule>::into).collect(),
        }
    }
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

impl From<u16> for PortRange {
    fn from(value: u16) -> Self {
        PortRange::port(value)
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
        self.0
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
            let start = start.parse::<u16>().map_err(D::Error::custom)?;
            let end = end.parse::<u16>().map_err(D::Error::custom)?;

            Ok(PortRange(RangeInclusive::new(start, end)))
        } else {
            let port = value.parse::<u16>().map_err(D::Error::custom)?;

            Ok(PortRange(RangeInclusive::new(port, port)))
        }
    }
}

/// Describes a single Firewall rule.
#[derive(Default, Clone, PartialEq, Eq, Debug, Deserialize)]
pub(crate) struct InternalRule {
    pub ip_version: Option<IPVersion>,
    pub name: String,
    pub dst_ip: Option<Ipv4Net>,
    pub src_ip: Option<Ipv4Net>,
    pub dst_port: Option<PortRange>,
    pub src_port: Option<PortRange>,
    pub protocol: Option<InternalProtocol>,
    pub tcp_flags: Option<String>,
    pub action: Action,
}

#[derive(Debug, Clone)]
pub enum Filter {
    Any(AnyFilter),
    Ipv4(Ipv4Filter),
    Ipv6(Ipv6Filter),
}

impl Default for Filter {
    fn default() -> Self {
        Filter::Any(AnyFilter::default())
    }
}

impl From<Ipv4Filter> for Filter {
    fn from(value: Ipv4Filter) -> Self {
        Filter::Ipv4(value)
    }
}

impl From<Ipv6Filter> for Filter {
    fn from(value: Ipv6Filter) -> Self {
        Filter::Ipv6(value)
    }
}

/// Filters both IPv4 and IPv6 traffic.
#[derive(Debug, Clone, Default)]
pub struct AnyFilter {
    /// Destination Port.
    pub dst_port: Option<PortRange>,

    /// Source Port.
    pub src_port: Option<PortRange>,
}

impl AnyFilter {
    /// Narrow filter to only match the given source port or port range.
    pub fn from_port<IntoPortRange: Into<PortRange>>(mut self, range: IntoPortRange) -> Self {
        self.src_port = Some(range.into());
        self
    }

    /// Narrow filter to only match the given destination port or port range.
    pub fn to_port<IntoPortRange: Into<PortRange>>(mut self, range: IntoPortRange) -> Self {
        self.dst_port = Some(range.into());
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct Ipv6Filter {
    /// Protocol.
    pub protocol: Option<Protocol>,

    /// Destination Port.
    pub dst_port: Option<PortRange>,

    /// Source Port.
    pub src_port: Option<PortRange>,
}

impl Ipv6Filter {
    /// Match all IPv6 traffic.
    pub fn any() -> Self {
        Ipv6Filter {
            protocol: None,
            dst_port: None,
            src_port: None,
        }
    }

    /// Match only IPSec Authentication Header traffic.
    pub fn ah() -> Self {
        Ipv6Filter {
            protocol: Some(Protocol::AH),
            dst_port: None,
            src_port: None,
        }
    }

    /// Match only IPSec Encapsulating Security Payload traffic.
    pub fn esp() -> Self {
        Ipv6Filter {
            protocol: Some(Protocol::ESP),
            dst_port: None,
            src_port: None,
        }
    }

    /// Match only IP-in-IP tunnelling traffic.
    pub fn ipip() -> Self {
        Ipv6Filter {
            protocol: Some(Protocol::IPIP),
            dst_port: None,
            src_port: None,
        }
    }

    /// Match only Generic Routing Encapsulation traffic.
    pub fn gre() -> Self {
        Ipv6Filter {
            protocol: Some(Protocol::GRE),
            dst_port: None,
            src_port: None,
        }
    }

    /// Match only User Datagram Protocol traffic.
    pub fn udp() -> Self {
        Ipv6Filter {
            protocol: Some(Protocol::UDP),
            dst_port: None,
            src_port: None,
        }
    }

    /// Match only Transmission Control Protocol traffic, optionally only with the given flags.
    pub fn tcp(flags: Option<String>) -> Self {
        Ipv6Filter {
            protocol: Some(Protocol::TCP { flags }),
            dst_port: None,
            src_port: None,
        }
    }

    /// Narrow filter to only match the given source port or port range.
    pub fn from_port<IntoPortRange: Into<PortRange>>(mut self, range: IntoPortRange) -> Self {
        self.src_port = Some(range.into());
        self
    }

    /// Narrow filter to only match the given destination port or port range.
    pub fn to_port<IntoPortRange: Into<PortRange>>(mut self, range: IntoPortRange) -> Self {
        self.dst_port = Some(range.into());
        self
    }
}

/// Filters IPv4 traffic.
#[derive(Debug, Clone, Default)]
pub struct Ipv4Filter {
    /// Destination IP address.
    pub dst_ip: Option<Ipv4Net>,

    /// Source IP address.
    ///
    /// Hetzner [does not support IPv6 address filtering](https://docs.hetzner.com/robot/dedicated-server/firewall#limitations-ipv6),
    /// hence why this is an [`Ipv4Net`], and not an [`IpNet`](ipnet::IpNet).
    pub src_ip: Option<Ipv4Net>,

    /// Destination Port.
    ///
    /// Hetzner [does not support IPv6 address filtering](https://docs.hetzner.com/robot/dedicated-server/firewall#limitations-ipv6),
    /// hence why this is an [`Ipv4Net`], and not an [`IpNet`](ipnet::IpNet).
    pub dst_port: Option<PortRange>,

    /// Source Port.
    pub src_port: Option<PortRange>,

    /// Protocol
    pub protocol: Option<Protocol>,
}

impl Ipv4Filter {
    /// Match all IPv4 traffic.
    pub fn any() -> Self {
        Ipv4Filter {
            protocol: None,
            dst_port: None,
            src_port: None,
            src_ip: None,
            dst_ip: None,
        }
    }

    /// Match only IPSec Authentication Header traffic.
    pub fn ah() -> Self {
        Ipv4Filter {
            protocol: Some(Protocol::AH),
            dst_port: None,
            src_port: None,
            src_ip: None,
            dst_ip: None,
        }
    }

    /// Match only IPSec Enapsulating Security Payload traffic.
    pub fn esp() -> Self {
        Ipv4Filter {
            protocol: Some(Protocol::ESP),
            dst_port: None,
            src_port: None,
            src_ip: None,
            dst_ip: None,
        }
    }

    /// Match only IP-in-IP tunnelling traffic.
    pub fn ipip() -> Self {
        Ipv4Filter {
            protocol: Some(Protocol::IPIP),
            dst_port: None,
            src_port: None,
            src_ip: None,
            dst_ip: None,
        }
    }

    /// Match only Generic Routing Encapsulation traffic.
    pub fn gre() -> Self {
        Ipv4Filter {
            protocol: Some(Protocol::GRE),
            dst_port: None,
            src_port: None,
            src_ip: None,
            dst_ip: None,
        }
    }

    /// Match only User Datagram Protocol traffic.
    pub fn udp() -> Self {
        Ipv4Filter {
            protocol: Some(Protocol::UDP),
            dst_port: None,
            src_port: None,
            src_ip: None,
            dst_ip: None,
        }
    }

    /// Match only Transmission Control Protocol traffic, optionally only the given flags.
    pub fn tcp(flags: Option<String>) -> Self {
        Ipv4Filter {
            protocol: Some(Protocol::TCP { flags }),
            dst_port: None,
            src_port: None,
            src_ip: None,
            dst_ip: None,
        }
    }

    /// Narrow filter to only match the given source port or port range.
    pub fn from_port<IntoPortRange: Into<PortRange>>(mut self, range: IntoPortRange) -> Self {
        self.src_port = Some(range.into());
        self
    }

    /// Narrow filter to only match the given destination port or port range.
    pub fn to_port<IntoPortRange: Into<PortRange>>(mut self, range: IntoPortRange) -> Self {
        self.dst_port = Some(range.into());
        self
    }

    /// Narrow filter to only match the given source ip address or address range.
    pub fn from_ip<IntoIpNet: Into<Ipv4Net>>(mut self, ip: IntoIpNet) -> Self {
        self.src_ip = Some(ip.into());
        self
    }

    /// Narrow filter to only match the given destination ip address or address range.
    pub fn to_ip<IntoIpNet: Into<Ipv4Net>>(mut self, ip: IntoIpNet) -> Self {
        self.dst_ip = Some(ip.into());
        self
    }
}

/// Describes a single firewall rule.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Human-readable name for the rule.
    pub name: String,

    /// Filter describing which traffic this rule applies to.
    pub filter: Filter,

    /// Action to take, if the filter matches.
    pub action: Action,
}

impl Rule {
    pub fn accept(name: &str) -> Self {
        Rule {
            name: name.to_string(),
            filter: Filter::default(),
            action: Action::Accept,
        }
    }

    pub fn discard(name: &str) -> Self {
        Rule {
            name: name.to_string(),
            filter: Filter::default(),
            action: Action::Discard,
        }
    }

    pub fn matching<F: Into<Filter>>(self, filter: F) -> Self {
        Rule {
            name: self.name,
            action: self.action,
            filter: filter.into(),
        }
    }
}

impl From<&Rule> for InternalRule {
    fn from(value: &Rule) -> Self {
        match &value.filter {
            Filter::Any(any) => InternalRule {
                name: value.name.clone(),
                src_port: any.src_port.clone(),
                dst_port: any.dst_port.clone(),
                action: value.action,
                ..Default::default()
            },
            Filter::Ipv4(ipv4) => InternalRule {
                ip_version: Some(IPVersion::IPv6),
                name: value.name.clone(),
                dst_port: ipv4.dst_port.clone(),
                src_port: ipv4.src_port.clone(),
                src_ip: ipv4.src_ip,
                dst_ip: ipv4.dst_ip,
                tcp_flags: ipv4.protocol.as_ref().and_then(Protocol::flags),
                protocol: ipv4.protocol.as_ref().map(Into::<InternalProtocol>::into),
                action: value.action,
            },
            Filter::Ipv6(ipv6) => InternalRule {
                ip_version: Some(IPVersion::IPv6),
                name: value.name.clone(),
                dst_port: ipv6.dst_port.clone(),
                src_port: ipv6.src_port.clone(),
                src_ip: None,
                dst_ip: None,
                tcp_flags: ipv6.protocol.as_ref().and_then(Protocol::flags),
                protocol: ipv6
                    .protocol
                    .as_ref()
                    .map(Into::<InternalProtocol>::into)
                    .clone(),
                action: value.action,
            },
        }
    }
}

impl From<InternalRule> for Rule {
    fn from(value: InternalRule) -> Self {
        let rule = match value.action {
            Action::Accept => Rule::accept(&value.name),
            Action::Discard => Rule::discard(&value.name),
        };

        let protocol = value.protocol.map(|protocol| match protocol {
            InternalProtocol::TCP => Protocol::TCP {
                flags: value.tcp_flags,
            },
            InternalProtocol::AH => Protocol::AH,
            InternalProtocol::ESP => Protocol::ESP,
            InternalProtocol::GRE => Protocol::GRE,
            InternalProtocol::ICMP => Protocol::ICMP,
            InternalProtocol::IPIP => Protocol::IPIP,
            InternalProtocol::UDP => Protocol::UDP,
        });

        rule.matching(match value.ip_version {
            Some(IPVersion::IPv4) => Filter::Ipv4(Ipv4Filter {
                dst_ip: value.dst_ip,
                src_ip: value.src_ip,
                dst_port: value.dst_port,
                src_port: value.src_port,
                protocol,
            }),
            Some(IPVersion::IPv6) => Filter::Ipv6(Ipv6Filter {
                dst_port: value.dst_port,
                src_port: value.src_port,
                protocol,
            }),
            None => Filter::Any(AnyFilter {
                dst_port: value.dst_port,
                src_port: value.src_port,
            }),
        })
    }
}

#[test]
fn build_rule() {
    let config = Rules {
        ingress: vec![
            Rule::accept("Allow all").matching(Ipv6Filter::udp().from_port(32768..=60000))
        ],
        egress: vec![],
    };
}

impl UrlEncode for InternalRule {
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

impl UrlEncode for InternalRules {
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

impl UrlEncode for InternalFirewallConfiguration {
    fn encode_into(&self, mut f: UrlEncodingBuffer<'_>) {
        f.set("status", self.status);
        f.set("filter_ipv6", self.filter_ipv6);
        f.set("whitelist_hos", self.whitelist_hetzner_services);
        self.rules.encode_into(f.append("rules"));
    }
}

impl UrlEncode for InternalFirewallTemplateConfiguration {
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

    use crate::models::{FirewallConfiguration, Ipv4Filter, Rule, Rules, State};

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
                    ingress: vec![
                        Rule::accept("HTTP").matching(Ipv4Filter::tcp(None).to_port(80)),
                        Rule::accept("HTTPS").matching(Ipv4Filter::tcp(None).to_port(443))
                    ],
                    egress: vec![Rule::accept("Allow all")],
                },
            }
            .encode()
        );
    }
}
