use clap::{Args, Subcommand};
use hrobot::AsyncRobot;
use tabled::Tabled;

#[derive(Args)]
pub struct RdnsArgs {
    #[command(subcommand)]
    command: RdnsCommand,
}

#[derive(Subcommand)]
enum RdnsCommand {
    /// List all reverse DNS entries
    List,
    /// Get reverse DNS entry for an IP
    Get {
        /// IP address
        ip: String,
    },
    /// Create reverse DNS entry
    Create {
        /// IP address
        ip: String,
        /// PTR record (hostname)
        ptr: String,
    },
    /// Update reverse DNS entry
    Update {
        /// IP address
        ip: String,
        /// New PTR record (hostname)
        ptr: String,
    },
    /// Delete reverse DNS entry
    Delete {
        /// IP address
        ip: String,
    },
}

#[derive(Tabled, serde::Serialize)]
struct RdnsListItem {
    #[tabled(rename = "IP")]
    ip: String,
    #[tabled(rename = "PTR")]
    ptr: String,
}

pub async fn execute(
    robot: AsyncRobot,
    args: RdnsArgs,
    output_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        RdnsCommand::List => {
            let entries = robot.list_rdns_entries().await?;
            let rdns_list: Vec<RdnsListItem> = entries
                .into_iter()
                .map(|e| RdnsListItem {
                    ip: e.ip.to_string(),
                    ptr: e.ptr,
                })
                .collect();

            crate::output::print_table_or_json(&rdns_list, output_format)?;
            Ok(())
        }
        RdnsCommand::Get { ip } => {
            let ip_addr: std::net::IpAddr = ip.parse()?;
            let entry = robot.get_rdns_entry(ip_addr).await?;
            let json = serde_json::to_string_pretty(&entry)?;
            println!("{}", json);
            Ok(())
        }
        RdnsCommand::Create { ip, ptr } => {
            let ip_addr: std::net::IpAddr = ip.parse()?;
            robot.create_rdns_entry(ip_addr, &ptr).await?;
            println!("Reverse DNS entry created: {} -> {}", ip, ptr);
            Ok(())
        }
        RdnsCommand::Update { ip, ptr } => {
            let ip_addr: std::net::IpAddr = ip.parse()?;
            robot.update_rdns_entry(ip_addr, &ptr).await?;
            println!("Reverse DNS entry updated: {} -> {}", ip, ptr);
            Ok(())
        }
        RdnsCommand::Delete { ip } => {
            let ip_addr: std::net::IpAddr = ip.parse()?;
            robot.delete_rdns_entry(ip_addr).await?;
            println!("Reverse DNS entry deleted for {}", ip);
            Ok(())
        }
    }
}
