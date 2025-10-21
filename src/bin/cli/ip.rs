use clap::{Args, Subcommand};
use hrobot::AsyncRobot;
use tabled::Tabled;

#[derive(Args)]
pub struct IpArgs {
    #[command(subcommand)]
    command: IpCommand,
}

#[derive(Subcommand)]
enum IpCommand {
    /// List all IP addresses
    List,
    /// Describe a specific IP address
    Describe {
        /// IP address
        ip: String,
    },
}

#[derive(Tabled, serde::Serialize)]
struct IpListItem {
    #[tabled(rename = "IP")]
    ip: String,
    #[tabled(rename = "SERVER NUMBER")]
    server_number: u32,
    #[tabled(rename = "LOCKED")]
    locked: bool,
    #[tabled(rename = "SEPARATE MAC")]
    separate_mac: String,
}

pub async fn execute(
    robot: AsyncRobot,
    args: IpArgs,
    output_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        IpCommand::List => {
            let ips = robot.list_ips().await?;
            let mut ip_list = Vec::new();

            for (server_id, server_ips) in ips {
                for ip in server_ips {
                    ip_list.push(IpListItem {
                        ip: ip.ip.to_string(),
                        server_number: server_id.0,
                        locked: ip.locked,
                        separate_mac: ip
                            .separate_mac
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| "-".to_string()),
                    });
                }
            }

            crate::output::print_table_or_json(&ip_list, output_format)?;
            Ok(())
        }
        IpCommand::Describe { ip } => {
            let ip_addr: std::net::Ipv4Addr = ip.parse()?;
            let ip_info = robot.get_ip(ip_addr).await?;
            let json = serde_json::to_string_pretty(&ip_info)?;
            println!("{}", json);
            Ok(())
        }
    }
}
