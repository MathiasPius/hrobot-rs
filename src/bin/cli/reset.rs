use clap::{Args, Subcommand};
use hrobot::{api::server::ServerId, AsyncRobot};

#[derive(Args)]
pub struct ResetArgs {
    #[command(subcommand)]
    command: ResetCommand,
}

#[derive(Subcommand)]
enum ResetCommand {
    /// List available reset options
    List,
    /// Get reset options for a specific server
    Get {
        /// Server number
        server_id: u32,
    },
    /// Trigger a hardware reset
    Trigger {
        /// Server number
        server_id: u32,
        /// Reset type (hw, sw, man, power, power_long)
        reset_type: String,
    },
}

pub async fn execute(
    robot: AsyncRobot,
    args: ResetArgs,
    _output_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        ResetCommand::List => {
            let reset_options = robot.list_reset_options().await?;
            let json = serde_json::to_string_pretty(&reset_options)?;
            println!("{}", json);
            Ok(())
        }
        ResetCommand::Get { server_id } => {
            let reset = robot.get_reset_options(ServerId(server_id)).await?;
            let json = serde_json::to_string_pretty(&reset)?;
            println!("{}", json);
            Ok(())
        }
        ResetCommand::Trigger {
            server_id,
            reset_type,
        } => {
            let reset_type = match reset_type.as_str() {
                "hw" => hrobot::api::reset::Reset::Hardware,
                "sw" => hrobot::api::reset::Reset::Software,
                "man" => hrobot::api::reset::Reset::Manual,
                "power" => hrobot::api::reset::Reset::Power,
                "power_long" => hrobot::api::reset::Reset::PowerLong,
                _ => return Err(format!("Invalid reset type: {}. Valid types: hw, sw, man, power, power_long", reset_type).into()),
            };

            robot.trigger_reset(ServerId(server_id), reset_type).await?;
            println!("Reset triggered for server {}", server_id);
            Ok(())
        }
    }
}
