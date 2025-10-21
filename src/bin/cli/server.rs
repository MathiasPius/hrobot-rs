use clap::{Args, Subcommand};
use hrobot::{api::server::ServerId, AsyncRobot};
use tabled::Tabled;

#[derive(Args)]
pub struct ServerArgs {
    #[command(subcommand)]
    command: ServerCommand,
}

#[derive(Subcommand)]
enum ServerCommand {
    /// List all servers
    List,
    /// Describe a specific server
    Describe {
        /// Server number
        server_id: u32,
    },
    /// Rename a server
    Rename {
        /// Server number
        server_id: u32,
        /// New name for the server
        name: String,
    },
    /// Show server cancellation status
    ShowCancellation {
        /// Server number
        server_id: u32,
    },
    /// Cancel a server
    Cancel {
        /// Server number
        server_id: u32,
        /// Cancellation date (YYYY-MM-DD, optional)
        #[arg(short, long)]
        date: Option<String>,
        /// Cancellation reason (optional)
        #[arg(short, long)]
        reason: Option<String>,
    },
    /// Withdraw server cancellation
    WithdrawCancellation {
        /// Server number
        server_id: u32,
    },
}

#[derive(Tabled, serde::Serialize)]
struct ServerListItem {
    #[tabled(rename = "SERVER ID")]
    server_id: u32,
    #[tabled(rename = "SERVER IP")]
    server_ip: String,
    #[tabled(rename = "NAME")]
    server_name: String,
    #[tabled(rename = "PRODUCT")]
    product: String,
    #[tabled(rename = "DC")]
    dc: String,
    #[tabled(rename = "STATUS")]
    status: String,
}

pub async fn execute(
    robot: AsyncRobot,
    args: ServerArgs,
    output_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        ServerCommand::List => {
            let servers = robot.list_servers().await?;
            let server_list: Vec<ServerListItem> = servers
                .into_iter()
                .map(|s| ServerListItem {
                    server_id: s.id.0,
                    server_ip: s.ipv4.map(|ip| ip.to_string()).unwrap_or_else(|| "-".to_string()),
                    server_name: s.name,
                    product: s.product,
                    dc: s.dc,
                    status: if s.cancelled { "cancelled" } else { "active" }.to_string(),
                })
                .collect();

            crate::output::print_table_or_json(&server_list, output_format)?;
            Ok(())
        }
        ServerCommand::Describe { server_id } => {
            let server = robot.get_server(ServerId(server_id)).await?;
            let json = serde_json::to_string_pretty(&server)?;
            println!("{}", json);
            Ok(())
        }
        ServerCommand::Rename { server_id, name } => {
            robot.rename_server(ServerId(server_id), &name).await?;
            println!("Server {} renamed to '{}'", server_id, name);
            Ok(())
        }
        ServerCommand::ShowCancellation { server_id } => {
            let cancellation = robot.get_server_cancellation(ServerId(server_id)).await?;
            let json = serde_json::to_string_pretty(&cancellation)?;
            println!("{}", json);
            Ok(())
        }
        ServerCommand::Cancel {
            server_id,
            date,
            reason,
        } => {
            // Parse the date if provided
            let parsed_date = date.map(|d| {
                time::Date::parse(
                    &d,
                    &time::format_description::parse("[year]-[month]-[day]").unwrap(),
                )
            }).transpose()?;

            let cancellation = hrobot::api::server::Cancel {
                date: parsed_date,
                reason,
                reserved: false,
            };

            robot.cancel_server(ServerId(server_id), cancellation).await?;
            println!("Server {} cancelled", server_id);
            Ok(())
        }
        ServerCommand::WithdrawCancellation { server_id } => {
            robot.withdraw_server_cancellation(ServerId(server_id)).await?;
            println!("Server {} cancellation withdrawn", server_id);
            Ok(())
        }
    }
}
