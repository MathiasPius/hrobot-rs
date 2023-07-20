use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::RangeInclusive};

pub use ipnet::Ipv4Net;

/// Unique Template ID.
///
/// Simple wrapper around a u32, to avoid confusion with for example [`ServerId`](crate::api::server::ServerId)
/// and to make it intuitive what kind of argument you need to give to functions like
/// [`AsyncRobot::get_firewall_template`](crate::AsyncRobot::get_firewall_template()).
///
/// Using a plain integer means it isn't clear what the argument is, is it a counter of my templates, where the argument
/// is in range `0..N` where `N` is the number of templates I have in my account, or is it a limiter, like get first `N`
/// templates, for example.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TemplateId(pub u32);

impl From<u32> for TemplateId {
    fn from(value: u32) -> Self {
        TemplateId(value)
    }
}

impl From<TemplateId> for u32 {
    fn from(value: TemplateId) -> Self {
        value.0
    }
}

impl Display for TemplateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<u32> for TemplateId {
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

/// Desired or current state of the server's firewall.
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
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Protocol {
    /// Transmission Control Protocol.
    Tcp { flags: Option<String> },

    /// User Datagram Protocol.
    Udp,

    /// Generic Routing Encapsulation.
    Gre,

    /// Internet Control Message Protocol.
    Icmp,

    /// IP-in-IP tunneling.
    Ipip,

    /// IPSec Authentication Header.
    Ah,

    /// IPSec Encapsulating Security Payload.
    Esp,
}

impl Protocol {
    pub fn tcp_with_flags(flags: &str) -> Self {
        Protocol::Tcp {
            flags: Some(flags.to_string()),
        }
    }

    pub(crate) fn flags(&self) -> Option<String> {
        match self {
            Protocol::Tcp { flags } => flags.clone(),
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
    pub id: TemplateId,

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
    pub id: TemplateId,

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

/// Desired configuration for a firewall template.
#[derive(Debug, Clone)]
pub struct FirewallTemplateConfig {
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
/// For configuring the firewall, instead use the [`FirewallConfig`] struct,
/// which can also be extracted using [`Firewall::config()`]
#[derive(Debug, Clone)]
pub struct Firewall {
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
    pub fn config(&self) -> FirewallConfig {
        self.into()
    }
}

/// Firewall configuration to apply to a server.
#[derive(Debug)]
pub struct FirewallConfig {
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

impl FirewallConfig {
    /// Transform into a template configuration, which can be used to create a template from.
    #[must_use = "This doesn't create the template, only produces a config which you can then upload with AsyncRobot::create_firewall_template"]
    pub fn to_template_config(&self, name: &str) -> FirewallTemplateConfig {
        FirewallTemplateConfig {
            name: name.to_string(),
            filter_ipv6: self.filter_ipv6,
            whitelist_hetzner_services: self.whitelist_hetzner_services,
            is_default: false,
            rules: self.rules.clone(),
        }
    }
}

impl From<&Firewall> for FirewallConfig {
    fn from(value: &Firewall) -> Self {
        FirewallConfig {
            status: value.status,
            filter_ipv6: value.filter_ipv6,
            whitelist_hetzner_services: value.whitelist_hetzner_services,
            rules: value.rules.clone(),
        }
    }
}

/// Encapsulates all ingoing and outgoing rules for a Firewall.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rules {
    /// Rules applied to ingress traffic (traffic to the server).
    pub ingress: Vec<Rule>,

    /// Rules applied to egress traffic (traffic leaving the server).
    pub egress: Vec<Rule>,
}

/// Describes a port or range of ports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortRange(RangeInclusive<u16>);

impl PortRange {
    /// Construct a port range covering only a single port.
    ///
    /// Equivalent to PortRange::range(port, port);
    ///
    /// # Example
    /// ```rust
    /// # use hrobot::api::firewall::PortRange;
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
    /// # use hrobot::api::firewall::PortRange;
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

/// Describes a filter which narrows the scope of affected traffic for a [`Rule`]
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, Default, PartialEq, Eq)]
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

/// Filters IPv6 traffic.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
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
            protocol: Some(Protocol::Ah),
            dst_port: None,
            src_port: None,
        }
    }

    /// Match only IPSec Encapsulating Security Payload traffic.
    pub fn esp() -> Self {
        Ipv6Filter {
            protocol: Some(Protocol::Esp),
            dst_port: None,
            src_port: None,
        }
    }

    /// Match only IP-in-IP tunnelling traffic.
    pub fn ipip() -> Self {
        Ipv6Filter {
            protocol: Some(Protocol::Ipip),
            dst_port: None,
            src_port: None,
        }
    }

    /// Match only Generic Routing Encapsulation traffic.
    pub fn gre() -> Self {
        Ipv6Filter {
            protocol: Some(Protocol::Gre),
            dst_port: None,
            src_port: None,
        }
    }

    /// Match only User Datagram Protocol traffic.
    pub fn udp() -> Self {
        Ipv6Filter {
            protocol: Some(Protocol::Udp),
            dst_port: None,
            src_port: None,
        }
    }

    /// Match only Transmission Control Protocol traffic, optionally only with the given flags.
    pub fn tcp(flags: Option<String>) -> Self {
        Ipv6Filter {
            protocol: Some(Protocol::Tcp { flags }),
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
#[derive(Debug, Clone, Default, PartialEq, Eq)]
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
            protocol: Some(Protocol::Ah),
            dst_port: None,
            src_port: None,
            src_ip: None,
            dst_ip: None,
        }
    }

    /// Match only IPSec Enapsulating Security Payload traffic.
    pub fn esp() -> Self {
        Ipv4Filter {
            protocol: Some(Protocol::Esp),
            dst_port: None,
            src_port: None,
            src_ip: None,
            dst_ip: None,
        }
    }

    /// Match only IP-in-IP tunnelling traffic.
    pub fn ipip() -> Self {
        Ipv4Filter {
            protocol: Some(Protocol::Ipip),
            dst_port: None,
            src_port: None,
            src_ip: None,
            dst_ip: None,
        }
    }

    /// Match only Generic Routing Encapsulation traffic.
    pub fn gre() -> Self {
        Ipv4Filter {
            protocol: Some(Protocol::Gre),
            dst_port: None,
            src_port: None,
            src_ip: None,
            dst_ip: None,
        }
    }

    /// Match only User Datagram Protocol traffic.
    pub fn udp() -> Self {
        Ipv4Filter {
            protocol: Some(Protocol::Udp),
            dst_port: None,
            src_port: None,
            src_ip: None,
            dst_ip: None,
        }
    }

    /// Match only Transmission Control Protocol traffic, optionally only the given flags.
    pub fn tcp(flags: Option<String>) -> Self {
        Ipv4Filter {
            protocol: Some(Protocol::Tcp { flags }),
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
