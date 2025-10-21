use clap::Args;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub active_context: Option<String>,
    #[serde(default)]
    pub contexts: HashMap<String, Context>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let expanded_path = shellexpand::tilde(path);
        let config_path = PathBuf::from(expanded_path.as_ref());

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let contents = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let expanded_path = shellexpand::tilde(path);
        let config_path = PathBuf::from(expanded_path.as_ref());

        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)?;
        fs::write(config_path, contents)?;
        Ok(())
    }

    pub fn get_context(&self, name: &str) -> Option<&Context> {
        self.contexts.get(name)
    }

    pub fn add_context(&mut self, name: String, context: Context) {
        self.contexts.insert(name, context);
    }

    pub fn remove_context(&mut self, name: &str) -> Option<Context> {
        self.contexts.remove(name)
    }

    pub fn set_active_context(&mut self, name: Option<String>) {
        self.active_context = name;
    }
}

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    command: ConfigCommand,
}

#[derive(clap::Subcommand)]
enum ConfigCommand {
    /// Show configuration file path
    Path,
    /// Show current configuration
    Show,
}

pub async fn execute(
    args: ConfigArgs,
    config: &Config,
    config_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        ConfigCommand::Path => {
            let expanded_path = shellexpand::tilde(config_path);
            println!("{}", expanded_path);
            Ok(())
        }
        ConfigCommand::Show => {
            let config_str = toml::to_string_pretty(config)?;
            println!("{}", config_str);
            Ok(())
        }
    }
}
