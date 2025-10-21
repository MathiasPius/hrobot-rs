use clap::{Args, Subcommand};

use crate::config::{Config, Context};

#[derive(Args)]
pub struct ContextArgs {
    #[command(subcommand)]
    command: ContextCommand,
}

#[derive(Subcommand)]
enum ContextCommand {
    /// List all contexts
    List,
    /// Create a new context
    Create {
        /// Name of the context
        name: String,
        /// Hetzner Robot username
        #[arg(short, long)]
        username: Option<String>,
        /// Hetzner Robot password
        #[arg(short, long)]
        password: Option<String>,
    },
    /// Delete a context
    Delete {
        /// Name of the context to delete
        name: String,
    },
    /// Set the active context
    Use {
        /// Name of the context to use
        name: String,
    },
    /// Show the current active context
    Current,
}

pub async fn execute(
    args: ContextArgs,
    config: &Config,
    config_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config.clone();

    match args.command {
        ContextCommand::List => {
            if config.contexts.is_empty() {
                println!("No contexts configured");
            } else {
                for (name, _) in &config.contexts {
                    let active = if config.active_context.as_ref() == Some(name) {
                        " (active)"
                    } else {
                        ""
                    };
                    println!("{}{}", name, active);
                }
            }
            Ok(())
        }
        ContextCommand::Create {
            name,
            username,
            password,
        } => {
            let username = match username {
                Some(u) => u,
                None => {
                    print!("Username: ");
                    use std::io::Write;
                    std::io::stdout().flush()?;
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    input.trim().to_string()
                }
            };

            let password = match password {
                Some(p) => p,
                None => {
                    let pw = rpassword::prompt_password("Password: ")?;
                    pw
                }
            };

            config.add_context(name.clone(), Context { username, password });

            // If this is the first context, make it active
            if config.contexts.len() == 1 {
                config.set_active_context(Some(name.clone()));
            }

            config.save(config_path)?;
            println!("Context '{}' created successfully", name);
            Ok(())
        }
        ContextCommand::Delete { name } => {
            if config.remove_context(&name).is_some() {
                // If we're deleting the active context, unset it
                if config.active_context.as_ref() == Some(&name) {
                    config.set_active_context(None);
                }
                config.save(config_path)?;
                println!("Context '{}' deleted successfully", name);
                Ok(())
            } else {
                Err(format!("Context '{}' not found", name).into())
            }
        }
        ContextCommand::Use { name } => {
            if config.get_context(&name).is_some() {
                config.set_active_context(Some(name.clone()));
                config.save(config_path)?;
                println!("Active context set to '{}'", name);
                Ok(())
            } else {
                Err(format!("Context '{}' not found", name).into())
            }
        }
        ContextCommand::Current => {
            match &config.active_context {
                Some(name) => println!("{}", name),
                None => println!("No active context set"),
            }
            Ok(())
        }
    }
}
