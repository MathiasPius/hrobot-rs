use clap::{Args, Subcommand};
use hrobot::AsyncRobot;
use tabled::Tabled;

#[derive(Args)]
pub struct StorageboxArgs {
    #[command(subcommand)]
    command: StorageboxCommand,
}

#[derive(Subcommand)]
enum StorageboxCommand {
    /// List all storage boxes
    List,
    /// Get storage box details
    Get {
        /// Storage box ID
        id: u32,
    },
    /// Rename a storage box
    Rename {
        /// Storage box ID
        id: u32,
        /// New name
        name: String,
    },
}

#[derive(Tabled, serde::Serialize)]
struct StorageboxListItem {
    #[tabled(rename = "ID")]
    id: u32,
    #[tabled(rename = "NAME")]
    name: String,
    #[tabled(rename = "PRODUCT")]
    product: String,
    #[tabled(rename = "LOCATION")]
    location: String,
    #[tabled(rename = "DISK QUOTA")]
    disk_quota: String,
}

pub async fn execute(
    robot: AsyncRobot,
    args: StorageboxArgs,
    output_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        StorageboxCommand::List => {
            let storageboxes = robot.list_storageboxes().await?;
            let storagebox_list: Vec<StorageboxListItem> = storageboxes
                .into_iter()
                .map(|sb| StorageboxListItem {
                    id: sb.id.0,
                    name: sb.name.clone(),
                    product: sb.product.clone(),
                    location: sb.location.clone(),
                    disk_quota: "-".to_string(), // Not available in reference, need full details
                })
                .collect();

            crate::output::print_table_or_json(&storagebox_list, output_format)?;
            Ok(())
        }
        StorageboxCommand::Get { id } => {
            let storagebox = robot.get_storagebox(id.into()).await?;
            let json = serde_json::to_string_pretty(&storagebox)?;
            println!("{}", json);
            Ok(())
        }
        StorageboxCommand::Rename { id, name } => {
            robot.rename_storagebox(id.into(), &name).await?;
            println!("Storage box {} renamed to '{}'", id, name);
            Ok(())
        }
    }
}
