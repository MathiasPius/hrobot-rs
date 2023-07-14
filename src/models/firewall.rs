use serde::{Deserialize, Serialize};
use std::{fmt::Display, net::Ipv4Addr};

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

impl Display for IPVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IPVersion::IPv4 => "ipv4",
                IPVersion::IPv6 => "ipv6",
            }
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
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
pub enum Port {
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

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Protocol::TCP => "tcp",
                Protocol::UDP => "udp",
                Protocol::GRE => "gre",
                Protocol::ICMP => "icmp",
                Protocol::IPIP => "ipip",
                Protocol::AH => "ah",
                Protocol::ESP => "esp",
            }
        )
    }
}

/// Course of action to take when a rule matches.
#[derive(Default, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    /// Explicitly accept the packet.
    #[default]
    Accept,

    /// Explicitly discard (or "drop") the packet.
    Discard,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Action::Accept => "accept",
                Action::Discard => "discard",
            }
        )
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
    pub port: Port,

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
    #[serde(rename = "input", default, skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<Rule>,

    #[serde(rename = "output", default, skip_serializing_if = "Vec::is_empty")]
    pub egress: Vec<Rule>,
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
    pub dst_port: Option<String>,

    /// Source Port.
    pub src_port: Option<String>,

    /// Protocol
    pub protocol: Option<Protocol>,

    /// TCP Flags.
    pub tcp_flags: Option<String>,

    /// Action to take if rule matches.
    pub action: Action,
}
