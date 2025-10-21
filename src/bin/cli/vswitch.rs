use clap::{Args, Subcommand};
use hrobot::AsyncRobot;
use tabled::Tabled;

#[derive(Args)]
pub struct VswitchArgs {
    #[command(subcommand)]
    command: VswitchCommand,
}

#[derive(Subcommand)]
enum VswitchCommand {
    /// List all vSwitches
    List,
    /// Get vSwitch details
    Get {
        /// vSwitch ID
        id: u32,
    },
}

#[derive(Tabled, serde::Serialize)]
struct VswitchListItem {
    #[tabled(rename = "ID")]
    id: u32,
    #[tabled(rename = "NAME")]
    name: String,
    #[tabled(rename = "VLAN")]
    vlan: u16,
    #[tabled(rename = "CANCELLED")]
    cancelled: bool,
}

pub async fn execute(
    robot: AsyncRobot,
    args: VswitchArgs,
    output_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        VswitchCommand::List => {
            let vswitches = robot.list_vswitches().await?;
            let vswitch_list: Vec<VswitchListItem> = vswitches
                .into_iter()
                .map(|vs| VswitchListItem {
                    id: vs.id.0,
                    name: vs.name,
                    vlan: vs.vlan.0,
                    cancelled: vs.cancelled,
                })
                .collect();

            crate::output::print_table_or_json(&vswitch_list, output_format)?;
            Ok(())
        }
        VswitchCommand::Get { id } => {
            let vswitch = robot.get_vswitch(id.into()).await?;
            println!("{:#?}", vswitch);
            Ok(())
        }
    }
}
