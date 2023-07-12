use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, net::Ipv4Addr};
use tracing::trace;

/// Version of the Internet Protocol supported by the firewall.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IPVersion {
    /// IPv4
    IPv4,
    /// IPv6
    IPv6,
}

impl Default for IPVersion {
    fn default() -> Self {
        IPVersion::IPv4
    }
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

#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Port {
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
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    /// Explicitly accept the packet.
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

/// Describes an entire Firewall for a server.
#[derive(Debug, Deserialize)]
pub struct Firewall {
    /// Primary IPv4 address of the server which this firewall applies to.
    #[serde(rename = "server_ip")]
    pub ipv4: Ipv4Addr,

    /// Unique server ID.
    #[serde(rename = "server_number")]
    pub id: u32,

    /// Status of the server's firewall.
    pub status: State,

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
    pub(crate) fn to_urlencoded(&self) -> Result<String, std::fmt::Error> {
        use std::fmt::Write;

        let mut segments = HashMap::new();

        fn serialize_rule(
            direction: &str,
            segments: &mut HashMap<String, String>,
            i: usize,
            rule: &Rule,
        ) {
            if let Some(ip_version) = rule.ip_version.as_ref() {
                segments.insert(
                    format!(
                        "rules[{direction}][{id}][{key}]",
                        id = i,
                        key = "ip_version"
                    ),
                    ip_version.to_string(),
                );
            }
            segments.insert(
                format!("rules[{direction}][{id}][{key}]", id = i, key = "name"),
                rule.name.to_owned(),
            );

            if let Some(dst_ip) = rule.dst_ip.as_ref() {
                segments.insert(
                    format!("rules[{direction}][{id}][{key}]", id = i, key = "dst_ip"),
                    dst_ip.to_owned(),
                );
            }

            if let Some(src_ip) = rule.src_ip.as_ref() {
                segments.insert(
                    format!("rules[{direction}][{id}][{key}]", id = i, key = "src_ip"),
                    src_ip.to_owned(),
                );
            }

            if let Some(dst_port) = rule.dst_port.as_ref() {
                segments.insert(
                    format!("rules[{direction}][{id}][{key}]", id = i, key = "dst_port"),
                    dst_port.to_owned(),
                );
            }

            if let Some(src_port) = rule.src_port.as_ref() {
                segments.insert(
                    format!("rules[{direction}][{id}][{key}]", id = i, key = "src_port"),
                    src_port.to_owned(),
                );
            }

            if let Some(protocol) = rule.protocol.as_ref() {
                segments.insert(
                    format!("rules[{direction}][{id}][{key}]", id = i, key = "protocol"),
                    protocol.to_string(),
                );
            }

            if let Some(tcp_flags) = rule.tcp_flags.as_ref() {
                segments.insert(
                    format!("rules[{direction}][{id}][{key}]", id = i, key = "tcp_flags"),
                    tcp_flags.to_owned(),
                );
            }

            segments.insert(
                format!("rules[{direction}][{id}][{key}]", id = i, key = "action"),
                rule.action.to_string(),
            );
        }

        for (index, rule) in self.rules.ingress.iter().enumerate() {
            serialize_rule("input", &mut segments, index, rule)
        }

        for (index, rule) in self.rules.egress.iter().enumerate() {
            serialize_rule("output", &mut segments, index, rule)
        }

        let mut segments: Vec<(_, _)> = segments.into_iter().collect();
        segments.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

        let mut query = String::new();

        write!(query, "status={}", self.status)?;
        write!(query, "&whitelist_hos={}", self.whitelist_hetzner_services)?;

        for (k, v) in segments.into_iter() {
            trace!("{k}={v}");
            write!(
                query,
                "&{}={}",
                urlencoding::encode(&k),
                urlencoding::encode(&v)
            )?;
        }

        Ok(query)
    }
}

/// Encapsulates all ingoing and outgoing rules for a Firewall.
#[derive(Debug, Deserialize, Serialize)]
pub struct Rules {
    #[serde(rename = "input", default, skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<Rule>,

    #[serde(rename = "output", default, skip_serializing_if = "Vec::is_empty")]
    pub egress: Vec<Rule>,
}

/// Describes a single Firewall rule.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_flags: Option<String>,

    /// Action to take if rule matches.
    pub action: Action,
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use tracing_test::traced_test;

    use crate::data::{IPVersion, Protocol};

    use super::{Firewall, Rule, Rules};

    #[test]
    #[traced_test]
    fn serializing_firewall_rules() {
        let firewall = Firewall {
            ipv4: Ipv4Addr::LOCALHOST,
            id: 123123123,
            status: super::State::Active,
            whitelist_hetzner_services: true,
            port: super::Port::Main,
            rules: Rules {
                ingress: vec![Rule {
                    ip_version: Some(IPVersion::IPv4),
                    name: "Allow all".to_owned(),
                    dst_ip: Some("127.0.0.0/8".to_string()),
                    src_ip: Some("0.0.0.0/0".to_string()),
                    dst_port: Some("27015-27016".to_string()),
                    src_port: None,
                    protocol: Some(Protocol::TCP),
                    tcp_flags: None,
                    action: super::Action::Accept,
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
                    action: super::Action::Accept,
                }],
            },
        };

        println!("{}", firewall.to_urlencoded().unwrap());
    }
}
