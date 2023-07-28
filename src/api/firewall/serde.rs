use std::fmt::Display;

use ipnet::Ipv4Net;
use serde::{Deserialize, Serialize};

use crate::urlencode::{UrlEncode, UrlEncodingBuffer};

use super::{
    Action, AnyFilter, Filter, Firewall, FirewallConfig, FirewallTemplate, FirewallTemplateConfig,
    Ipv4Filter, Ipv6Filter, PortRange, Protocol, Rule, Rules, State, SwitchPort, TemplateId,
};

/// Describes an entire firewall template.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct InternalFirewallTemplate {
    pub id: TemplateId,
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
pub(crate) struct InternalFirewallTemplateConfig {
    pub name: String,
    pub filter_ipv6: bool,
    pub whitelist_hetzner_services: bool,
    pub is_default: bool,
    pub rules: InternalRules,
}

impl From<FirewallTemplateConfig> for InternalFirewallTemplateConfig {
    fn from(value: FirewallTemplateConfig) -> Self {
        InternalFirewallTemplateConfig {
            name: value.name,
            filter_ipv6: value.filter_ipv6,
            whitelist_hetzner_services: value.whitelist_hetzner_services,
            is_default: value.is_default,
            rules: (&value.rules).into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct InternalFirewall {
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
            status: value.status,
            filter_ipv6: value.filter_ipv6,
            whitelist_hetzner_services: value.whitelist_hetzner_services,
            port: value.port,
            rules: value.rules.into(),
        }
    }
}

pub(crate) struct InternalFirewallConfig {
    pub status: State,
    pub filter_ipv6: bool,
    pub whitelist_hetzner_services: bool,
    pub rules: InternalRules,
}

impl From<&FirewallConfig> for InternalFirewallConfig {
    fn from(value: &FirewallConfig) -> Self {
        InternalFirewallConfig {
            status: value.status,
            filter_ipv6: value.filter_ipv6,
            whitelist_hetzner_services: value.whitelist_hetzner_services,
            rules: (&value.rules).into(),
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

/// Describes a single Firewall rule.
#[derive(Default, Clone, Debug, Deserialize)]
pub(crate) struct InternalRule {
    pub ip_version: Option<IpVersion>,
    pub name: String,
    pub dst_ip: Option<Ipv4Net>,
    pub src_ip: Option<Ipv4Net>,
    pub dst_port: Option<PortRange>,
    pub src_port: Option<PortRange>,
    pub protocol: Option<InternalProtocol>,
    pub tcp_flags: Option<String>,
    pub action: Action,
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
                ip_version: Some(IpVersion::Ipv4),
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
                ip_version: Some(IpVersion::Ipv6),
                name: value.name.clone(),
                dst_port: ipv6.dst_port.clone(),
                src_port: ipv6.src_port.clone(),
                src_ip: None,
                dst_ip: None,
                tcp_flags: ipv6.protocol.as_ref().and_then(Protocol::flags),
                protocol: ipv6.protocol.as_ref().map(Into::<InternalProtocol>::into),
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
            InternalProtocol::Tcp => Protocol::Tcp {
                flags: value.tcp_flags,
            },
            InternalProtocol::Ah => Protocol::Ah,
            InternalProtocol::Esp => Protocol::Esp,
            InternalProtocol::Gre => Protocol::Gre,
            InternalProtocol::Icmp => Protocol::Icmp,
            InternalProtocol::Ipip => Protocol::Ipip,
            InternalProtocol::Udp => Protocol::Udp,
        });

        rule.matching(match value.ip_version {
            Some(IpVersion::Ipv4) => Filter::Ipv4(Ipv4Filter {
                dst_ip: value.dst_ip,
                src_ip: value.src_ip,
                dst_port: value.dst_port,
                src_port: value.src_port,
                protocol,
            }),
            Some(IpVersion::Ipv6) => Filter::Ipv6(Ipv6Filter {
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

/// Version of the Internet Protocol supported by the firewall.
#[derive(Default, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum IpVersion {
    /// IPv4
    #[default]
    Ipv4,
    /// IPv6
    Ipv6,
}

impl AsRef<str> for IpVersion {
    fn as_ref(&self) -> &str {
        match self {
            IpVersion::Ipv4 => "ipv4",
            IpVersion::Ipv6 => "ipv6",
        }
    }
}

impl Display for IpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

/// Protocol types which can be used by rules.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum InternalProtocol {
    /// Transmission Control Protocol.
    Tcp,

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

impl AsRef<str> for InternalProtocol {
    fn as_ref(&self) -> &str {
        match self {
            InternalProtocol::Tcp => "tcp",
            InternalProtocol::Udp => "udp",
            InternalProtocol::Gre => "gre",
            InternalProtocol::Icmp => "icmp",
            InternalProtocol::Ipip => "ipip",
            InternalProtocol::Ah => "ah",
            InternalProtocol::Esp => "esp",
        }
    }
}

impl Display for InternalProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl From<&Protocol> for InternalProtocol {
    fn from(value: &Protocol) -> Self {
        match value {
            Protocol::Tcp { .. } => InternalProtocol::Tcp,
            Protocol::Udp => InternalProtocol::Udp,
            Protocol::Gre => InternalProtocol::Gre,
            Protocol::Icmp => InternalProtocol::Icmp,
            Protocol::Ipip => InternalProtocol::Ipip,
            Protocol::Ah => InternalProtocol::Ah,
            Protocol::Esp => InternalProtocol::Esp,
        }
    }
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
            for (index, rule) in self.egress.iter().enumerate() {
                rule.encode_into(egress.append(&format!("[{index}]")));
            }
        }
    }
}

impl UrlEncode for InternalFirewallConfig {
    fn encode_into(&self, mut f: UrlEncodingBuffer<'_>) {
        f.set("status", self.status);
        f.set("filter_ipv6", self.filter_ipv6);
        f.set("whitelist_hos", self.whitelist_hetzner_services);
        self.rules.encode_into(f.append("rules"));
    }
}

impl UrlEncode for InternalFirewallTemplateConfig {
    fn encode_into(&self, mut f: UrlEncodingBuffer<'_>) {
        f.set("name", &self.name);
        f.set("filter_ipv6", self.filter_ipv6);
        f.set("whitelist_hos", self.whitelist_hetzner_services);
        f.set("is_default", self.is_default);
        self.rules.encode_into(f.append("rules"));
    }
}
