use clap::{Args, Subcommand};
use hrobot::AsyncRobot;
use tabled::Tabled;

#[derive(Args)]
pub struct SubnetArgs {
    #[command(subcommand)]
    command: SubnetCommand,
}

#[derive(Subcommand)]
enum SubnetCommand {
    /// List all subnets
    List,
    /// Get subnet details
    Get {
        /// Subnet address (CIDR notation, e.g., 192.0.2.0/24)
        subnet: String,
    },
}

#[derive(Tabled, serde::Serialize)]
struct SubnetListItem {
    #[tabled(rename = "SUBNET")]
    subnet: String,
    #[tabled(rename = "SERVER IP")]
    server_ip: String,
    #[tabled(rename = "GATEWAY")]
    gateway: String,
    #[tabled(rename = "MASK")]
    mask: u8,
}

pub async fn execute(
    robot: AsyncRobot,
    args: SubnetArgs,
    output_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        SubnetCommand::List => {
            let subnets = robot.list_subnets().await?;
            let mut subnet_list = Vec::new();

            for (server_id, server_subnets) in subnets {
                for subnet in server_subnets {
                    subnet_list.push(SubnetListItem {
                        subnet: subnet.ip.to_string(),
                        server_ip: server_id.0.to_string(),
                        gateway: subnet.gateway.to_string(),
                        mask: subnet.ip.prefix_len(),
                    });
                }
            }

            crate::output::print_table_or_json(&subnet_list, output_format)?;
            Ok(())
        }
        SubnetCommand::Get { subnet } => {
            let subnet_addr: ipnet::IpNet = subnet.parse()?;
            let ip_addr = subnet_addr.addr();
            let subnet_info = robot.get_subnet(ip_addr).await?;
            let json = serde_json::to_string_pretty(&subnet_info)?;
            println!("{}", json);
            Ok(())
        }
    }
}
