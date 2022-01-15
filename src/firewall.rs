use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Write},
    net::Ipv4Addr,
};

use crate::{Error, SyncRobot};

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IPVersion {
    IPv4,
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
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "in process")]
    InProcess,
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Port {
    Main,
    Kvm,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    TCP,
    UDP,
    GRE,
    ICMP,
    IPIP,
    AH,
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

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Accept,
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

#[derive(Debug, Deserialize)]
pub struct Firewall {
    pub server_ip: Ipv4Addr,
    pub server_number: u32,
    pub status: State,
    pub whitelist_hos: bool,
    pub port: Port,
    pub rules: Rules,
}

#[derive(Debug, Deserialize)]
struct FirewallResponse {
    pub firewall: Firewall,
}

impl From<FirewallResponse> for Firewall {
    fn from(f: FirewallResponse) -> Self {
        f.firewall
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rules {
    pub input: Vec<Rule>,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Rule {
    pub ip_version: IPVersion,
    pub name: String,
    pub dst_ip: Option<String>,
    pub src_ip: Option<String>,
    pub dst_port: Option<String>,
    pub src_port: Option<String>,
    pub protocol: Option<Protocol>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_flags: Option<String>,
    pub action: Action,
}

struct SetFirewallRequest {
    status: State,
    whitelist_hos: Option<bool>,
    rules: Option<Rules>,
}

impl SetFirewallRequest {
    pub fn into_urlencoded(self) -> Result<String, std::fmt::Error> {
        let mut segments = HashMap::new();

        if let Some(rules) = self.rules {
            for (i, rule) in rules.input.into_iter().enumerate() {
                segments.insert(
                    format!("rules[input][{id}][{key}]", id = i, key = "ip_version"),
                    rule.ip_version.to_string(),
                );
                segments.insert(
                    format!("rules[input][{id}][{key}]", id = i, key = "name"),
                    rule.name,
                );

                if let Some(dst_ip) = rule.dst_ip {
                    segments.insert(
                        format!("rules[input][{id}][{key}]", id = i, key = "dst_ip"),
                        dst_ip,
                    );
                }

                if let Some(src_ip) = rule.src_ip {
                    segments.insert(
                        format!("rules[input][{id}][{key}]", id = i, key = "src_ip"),
                        src_ip,
                    );
                }

                if let Some(dst_port) = rule.dst_port {
                    segments.insert(
                        format!("rules[input][{id}][{key}]", id = i, key = "dst_port"),
                        dst_port,
                    );
                }

                if let Some(src_port) = rule.src_port {
                    segments.insert(
                        format!("rules[input][{id}][{key}]", id = i, key = "src_port"),
                        src_port,
                    );
                }

                if let Some(protocol) = rule.protocol {
                    segments.insert(
                        format!("rules[input][{id}][{key}]", id = i, key = "protocol"),
                        protocol.to_string(),
                    );
                }

                if let Some(tcp_flags) = rule.tcp_flags {
                    segments.insert(
                        format!("rules[input][{id}][{key}]", id = i, key = "tcp_flags"),
                        tcp_flags,
                    );
                }

                segments.insert(
                    format!("rules[input][{id}][{key}]", id = i, key = "action"),
                    rule.action.to_string(),
                );
            }
        }

        let mut segments: Vec<(_, _)> = segments.into_iter().collect();
        segments.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

        let mut query = String::new();

        write!(query, "status={}", self.status)?;
        if let Some(whitelist) = self.whitelist_hos {
            write!(query, "&whitelist_hos={}", whitelist)?;
        }

        for (k, v) in segments.into_iter() {
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

pub trait FirewallRobot {
    fn get_firewall(&self, server_number: u32) -> Result<Firewall, Error>;
    fn set_firewall_rules(
        &self,
        server_number: u32,
        rules: Option<Rules>,
        whitelist_hos: Option<bool>,
        state: State,
    ) -> Result<Firewall, Error>;
}

impl<T> FirewallRobot for T
where
    T: SyncRobot,
{
    fn get_firewall(&self, server_number: u32) -> Result<Firewall, Error> {
        self.get::<FirewallResponse>(&format!("/firewall/{}", server_number))
            .map(Firewall::from)
    }

    fn set_firewall_rules(
        &self,
        server_number: u32,
        rules: Option<Rules>,
        whitelist_hos: Option<bool>,
        status: State,
    ) -> Result<Firewall, Error> {
        let request = SetFirewallRequest {
            status,
            whitelist_hos,
            rules,
        };

        self.post_raw::<FirewallResponse>(
            &format!("/firewall/{}", server_number),
            request.into_urlencoded().unwrap(),
        )
        .map(Firewall::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Robot, ServerRobot};

    #[test]
    #[ignore]
    fn get_firewall() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);

        let firewall = robot.get_firewall(servers[0].server_number).unwrap();
        println!("{:#?}", firewall);
    }

    #[test]
    #[ignore]
    fn set_firewall() {
        let robot = Robot::default();

        let servers = robot.list_servers().unwrap();
        assert!(servers.len() > 0);

        println!("{:#?}", servers[0]);

        let firewall = robot.get_firewall(servers[0].server_number).unwrap();
        println!("{:#?}", firewall);

        let new_firewall = robot
            .set_firewall_rules(
                servers[0].server_number,
                Some(firewall.rules),
                Some(firewall.whitelist_hos),
                firewall.status,
            )
            .unwrap();

        println!("{:#?}", new_firewall);
    }
}
