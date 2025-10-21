use clap::{Args, Subcommand};
use hrobot::{api::server::ServerId, AsyncRobot};

#[derive(Args)]
pub struct BootArgs {
    #[command(subcommand)]
    command: BootCommand,
}

#[derive(Subcommand)]
enum BootCommand {
    /// Rescue system configuration
    #[command(subcommand)]
    Rescue(RescueCommand),
}

#[derive(Subcommand)]
enum RescueCommand {
    /// Get rescue configuration
    Get {
        /// Server number
        server_id: u32,
    },
    /// Enable rescue system
    Enable {
        /// Server number
        server_id: u32,
    },
    /// Disable rescue system
    Disable {
        /// Server number
        server_id: u32,
    },
}

pub async fn execute(
    robot: AsyncRobot,
    args: BootArgs,
    _output_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        BootCommand::Rescue(rescue_cmd) => match rescue_cmd {
            RescueCommand::Get { server_id } => {
                let rescue = robot.get_rescue_config(ServerId(server_id)).await?;
                let json = serde_json::to_string_pretty(&rescue)?;
                println!("{}", json);
                Ok(())
            }
            RescueCommand::Enable { server_id } => {
                use hrobot::api::boot::{RescueConfig, RescueOperatingSystem, Keyboard};
                // Use default rescue config (Linux 64-bit)
                let config = RescueConfig {
                    operating_system: RescueOperatingSystem::from("linux"),
                    authorized_keys: vec![],
                    keyboard: Keyboard::US,
                };
                let rescue = robot.enable_rescue_config(ServerId(server_id), config).await?;
                println!("Rescue system enabled for server {}", server_id);
                if let Some(password) = rescue.password {
                    println!("Root password: {}", password);
                }
                Ok(())
            }
            RescueCommand::Disable { server_id } => {
                robot.disable_rescue_config(ServerId(server_id)).await?;
                println!("Rescue system disabled for server {}", server_id);
                Ok(())
            }
        },
    }
}
