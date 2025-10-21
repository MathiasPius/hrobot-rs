use clap::{Args, Subcommand};
use hrobot::AsyncRobot;
use tabled::Tabled;

#[derive(Args)]
pub struct FailoverArgs {
    #[command(subcommand)]
    command: FailoverCommand,
}

#[derive(Subcommand)]
enum FailoverCommand {
    /// List all failover IPs
    List,
    /// Get failover IP details
    Get {
        /// Failover IP address
        ip: String,
    },
    /// Route failover IP to a server
    Route {
        /// Failover IP address
        ip: String,
        /// Target server number
        server_id: u32,
    },
    /// Disable failover routing
    Disable {
        /// Failover IP address
        ip: String,
    },
}

#[derive(Tabled, serde::Serialize)]
struct FailoverListItem {
    #[tabled(rename = "IP")]
    ip: String,
    #[tabled(rename = "ACTIVE SERVER")]
    active_server: u32,
    #[tabled(rename = "SERVER ADDRESS")]
    server_address: String,
}

pub async fn execute(
    robot: AsyncRobot,
    args: FailoverArgs,
    output_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        FailoverCommand::List => {
            let failovers = robot.list_failover_ips().await?;
            let failover_list: Vec<FailoverListItem> = failovers
                .into_iter()
                .map(|f| FailoverListItem {
                    ip: f.ip.to_string(),
                    active_server: f.active_server.0,
                    server_address: f.server_address.map(|a| a.to_string()).unwrap_or_else(|| "-".to_string()),
                })
                .collect();

            crate::output::print_table_or_json(&failover_list, output_format)?;
            Ok(())
        }
        FailoverCommand::Get { ip } => {
            let ip_addr: std::net::IpAddr = ip.parse()?;
            let failover = robot.get_failover_ip(ip_addr).await?;
            println!("{:#?}", failover);
            Ok(())
        }
        FailoverCommand::Route { ip, server_id } => {
            let ip_addr: std::net::IpAddr = ip.parse()?;
            // First get the server to find its IP address
            let server = robot.get_server(hrobot::api::server::ServerId(server_id)).await?;
            let target_ip = server.ipv4.ok_or("Server has no IPv4 address")?;
            robot.switch_failover_routing(ip_addr, target_ip.into()).await?;
            println!("Failover IP {} routed to server {}", ip, server_id);
            Ok(())
        }
        FailoverCommand::Disable { ip } => {
            let ip_addr: std::net::IpAddr = ip.parse()?;
            robot.disable_failover_routing(ip_addr).await?;
            println!("Failover routing disabled for {}", ip);
            Ok(())
        }
    }
}
