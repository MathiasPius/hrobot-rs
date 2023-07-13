use std::collections::HashMap;

use serde::ser::Error;
use tracing::trace;

use crate::models::{Firewall, FirewallConfiguration, Rule};

use super::{wrapper::Single, UnauthenticatedRequest};

pub(crate) fn get_firewall(server_number: u32) -> UnauthenticatedRequest<Single<Firewall>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/{server_number}"
    ))
}

pub(crate) fn set_firewall(
    server_number: u32,
    firewall: &FirewallConfiguration,
) -> Result<UnauthenticatedRequest<Single<Firewall>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/{server_number}"
    ))
    .with_method("POST")
    .with_body(urlencode_firewall(firewall).map_err(|_| {
        serde_html_form::ser::Error::custom(
            "formatting error while serializing firewall configuration",
        )
    })?)
}

fn urlencode_firewall(firewall: &FirewallConfiguration) -> Result<String, std::fmt::Error> {
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

    for (index, rule) in firewall.rules.ingress.iter().enumerate() {
        serialize_rule("input", &mut segments, index, rule)
    }

    for (index, rule) in firewall.rules.egress.iter().enumerate() {
        serialize_rule("output", &mut segments, index, rule)
    }

    let mut segments: Vec<(_, _)> = segments.into_iter().collect();
    segments.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

    let mut query = String::new();

    write!(query, "status={}", firewall.status)?;
    write!(
        query,
        "&\nwhitelist_hos={}",
        firewall.whitelist_hetzner_services
    )?;

    for (k, v) in segments.into_iter() {
        trace!("{k}={v}");
        write!(
            query,
            "&\n{}={}",
            urlencoding::encode(&k),
            urlencoding::encode(&v)
        )?;
    }

    Ok(query)
}

#[cfg(all(test, feature = "hyper-client"))]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    use crate::{
        api::firewall::urlencode_firewall,
        models::{Action, FirewallConfiguration, IPVersion, Protocol, Rule, Rules, State},
    };

    #[tokio::test]
    #[traced_test]
    async fn test_get_firewall() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.iter().next() {
            let firewall = robot.get_firewall(server.id).await.unwrap();

            info!("{firewall:#?}");
        }
    }

    #[test]
    #[traced_test]
    fn serializing_firewall_rules() {
        let firewall = FirewallConfiguration {
            status: State::Active,
            filter_ipv6: false,
            whitelist_hetzner_services: true,
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
        };

        println!("{}", urlencode_firewall(&firewall).unwrap());
    }
}
