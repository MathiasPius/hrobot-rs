use clap::{Args, Subcommand};
use hrobot::{api::server::ServerId, AsyncRobot};
use tabled::Tabled;

#[derive(Args)]
pub struct FirewallArgs {
    #[command(subcommand)]
    command: FirewallCommand,
}

#[derive(Subcommand)]
enum FirewallCommand {
    /// Get firewall configuration for a server
    Get {
        /// Server number
        server_id: u32,
    },
    /// Delete firewall configuration for a server
    Delete {
        /// Server number
        server_id: u32,
    },
    /// Apply a firewall template to a server
    ApplyTemplate {
        /// Server number
        server_id: u32,
        /// Template ID
        template_id: u32,
    },
    /// List firewall templates
    ListTemplates,
    /// Get a specific firewall template
    GetTemplate {
        /// Template ID
        template_id: u32,
    },
    /// Delete a firewall template
    DeleteTemplate {
        /// Template ID
        template_id: u32,
    },
}

#[derive(Tabled, serde::Serialize)]
struct TemplateListItem {
    #[tabled(rename = "ID")]
    id: u32,
    #[tabled(rename = "NAME")]
    name: String,
    #[tabled(rename = "WHITELIST")]
    is_whitelist: bool,
}

pub async fn execute(
    robot: AsyncRobot,
    args: FirewallArgs,
    output_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        FirewallCommand::Get { server_id } => {
            let firewall = robot.get_firewall(ServerId(server_id)).await?;
            println!("{:#?}", firewall);
            Ok(())
        }
        FirewallCommand::Delete { server_id } => {
            robot.delete_firewall(ServerId(server_id)).await?;
            println!("Firewall configuration deleted for server {}", server_id);
            Ok(())
        }
        FirewallCommand::ApplyTemplate {
            server_id,
            template_id,
        } => {
            robot
                .apply_firewall_template(ServerId(server_id), template_id.into())
                .await?;
            println!(
                "Firewall template {} applied to server {}",
                template_id, server_id
            );
            Ok(())
        }
        FirewallCommand::ListTemplates => {
            let templates = robot.list_firewall_templates().await?;
            let template_list: Vec<TemplateListItem> = templates
                .into_iter()
                .map(|t| TemplateListItem {
                    id: t.id.0,
                    name: t.name,
                    is_whitelist: t.whitelist_hetzner_services,
                })
                .collect();

            crate::output::print_table_or_json(&template_list, output_format)?;
            Ok(())
        }
        FirewallCommand::GetTemplate { template_id } => {
            let template = robot.get_firewall_template(template_id.into()).await?;
            println!("{:#?}", template);
            Ok(())
        }
        FirewallCommand::DeleteTemplate { template_id } => {
            robot.delete_firewall_template(template_id.into()).await?;
            println!("Firewall template {} deleted", template_id);
            Ok(())
        }
    }
}
