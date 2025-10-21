use clap::{Args, Subcommand};
use hrobot::AsyncRobot;
use tabled::Tabled;

#[derive(Args)]
pub struct SshKeyArgs {
    #[command(subcommand)]
    command: SshKeyCommand,
}

#[derive(Subcommand)]
enum SshKeyCommand {
    /// List all SSH keys
    List,
    /// Describe a specific SSH key
    Describe {
        /// SSH key fingerprint
        fingerprint: String,
    },
    /// Create a new SSH key
    Create {
        /// Name for the SSH key
        #[arg(short, long)]
        name: String,
        /// Public key data
        #[arg(short, long)]
        key: Option<String>,
        /// Read key from file
        #[arg(short, long)]
        file: Option<String>,
    },
    /// Rename an SSH key
    Rename {
        /// SSH key fingerprint
        fingerprint: String,
        /// New name for the SSH key
        name: String,
    },
    /// Delete an SSH key
    Delete {
        /// SSH key fingerprint
        fingerprint: String,
    },
}

#[derive(Tabled, serde::Serialize)]
struct SshKeyListItem {
    #[tabled(rename = "NAME")]
    name: String,
    #[tabled(rename = "FINGERPRINT")]
    fingerprint: String,
    #[tabled(rename = "TYPE")]
    key_type: String,
    #[tabled(rename = "SIZE")]
    size: u16,
}

pub async fn execute(
    robot: AsyncRobot,
    args: SshKeyArgs,
    output_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        SshKeyCommand::List => {
            let keys = robot.list_ssh_keys().await?;
            let key_list: Vec<SshKeyListItem> = keys
                .into_iter()
                .map(|k| SshKeyListItem {
                    name: k.name,
                    fingerprint: k.fingerprint,
                    key_type: k.algorithm,
                    size: k.bits,
                })
                .collect();

            crate::output::print_table_or_json(&key_list, output_format)?;
            Ok(())
        }
        SshKeyCommand::Describe { fingerprint } => {
            let key = robot.get_ssh_key(&fingerprint).await?;
            let json = serde_json::to_string_pretty(&key)?;
            println!("{}", json);
            Ok(())
        }
        SshKeyCommand::Create { name, key, file } => {
            let key_data = match (key, file) {
                (Some(k), None) => k,
                (None, Some(f)) => std::fs::read_to_string(f)?,
                (Some(_), Some(_)) => {
                    return Err("Cannot specify both --key and --file".into());
                }
                (None, None) => {
                    return Err("Must specify either --key or --file".into());
                }
            };

            let created_key = robot.create_ssh_key(&name, &key_data).await?;
            println!("SSH key '{}' created successfully", created_key.name);
            println!("Fingerprint: {}", created_key.fingerprint);
            Ok(())
        }
        SshKeyCommand::Rename { fingerprint, name } => {
            robot.rename_ssh_key(&fingerprint, &name).await?;
            println!("SSH key {} renamed to '{}'", fingerprint, name);
            Ok(())
        }
        SshKeyCommand::Delete { fingerprint } => {
            robot.remove_ssh_key(&fingerprint).await?;
            println!("SSH key {} deleted successfully", fingerprint);
            Ok(())
        }
    }
}
