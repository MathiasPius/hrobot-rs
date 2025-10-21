use clap::{Parser, Subcommand};
use std::process;

mod cli;
mod output;

use cli::*;
use cli::config::Config;

#[derive(Parser)]
#[command(
    name = "hrobot",
    about = "A command-line interface for Hetzner Robot",
    version,
    long_about = None
)]
struct Cli {
    /// Config file path
    #[arg(long, env = "HROBOT_CONFIG", default_value = "~/.config/hrobot/cli.toml")]
    config: String,

    /// Currently active context
    #[arg(long, env = "HROBOT_CONTEXT")]
    context: Option<String>,

    /// Enable debug output
    #[arg(long)]
    debug: bool,

    /// If true, only print error messages
    #[arg(long)]
    quiet: bool,

    /// Output format (table, json)
    #[arg(long, short = 'o', default_value = "table")]
    output: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage servers
    #[command(alias = "servers")]
    Server(server::ServerArgs),

    /// Manage IP addresses
    #[command(alias = "ips")]
    Ip(ip::IpArgs),

    /// Manage subnets
    #[command(alias = "subnets")]
    Subnet(subnet::SubnetArgs),

    /// Manage SSH keys
    #[command(alias = "ssh-keys")]
    SshKey(ssh_key::SshKeyArgs),

    /// Manage firewalls
    #[command(alias = "firewalls")]
    Firewall(firewall::FirewallArgs),

    /// Manage reverse DNS
    Rdns(rdns::RdnsArgs),

    /// Manage failover IPs
    #[command(alias = "failover-ips")]
    Failover(failover::FailoverArgs),

    /// Manage hardware resets
    Reset(reset::ResetArgs),

    /// Manage boot configurations
    Boot(boot::BootArgs),

    /// Manage storage boxes
    #[command(alias = "storageboxes")]
    Storagebox(storagebox::StorageboxArgs),

    /// Manage vSwitches
    #[command(alias = "vswitches")]
    Vswitch(vswitch::VswitchArgs),

    /// Manage configuration
    Config(config::ConfigArgs),

    /// Manage contexts
    Context(context::ContextArgs),

    /// Output shell completion code for the specified shell
    Completion(completion::CompletionArgs),

    /// Print version information
    Version,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize logging based on debug flag
    if cli.debug {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
    } else if !cli.quiet {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    // Load configuration
    let config = match Config::load(&cli.config) {
        Ok(cfg) => cfg,
        Err(e) => {
            if !cli.quiet {
                eprintln!("Warning: Failed to load config: {}", e);
                eprintln!("Using default configuration. You can create a config with 'hrobot context create'");
            }
            Config::default()
        }
    };

    // Determine which context to use
    let context_name = cli.context.as_ref().or(config.active_context.as_ref());

    // Get credentials from context or environment
    let (username, password) = if let Some(ctx_name) = context_name {
        match config.get_context(ctx_name) {
            Some(ctx) => (ctx.username.clone(), ctx.password.clone()),
            None => {
                eprintln!("Error: Context '{}' not found", ctx_name);
                process::exit(1);
            }
        }
    } else {
        // Try to get from environment variables
        match (
            std::env::var("HROBOT_USERNAME"),
            std::env::var("HROBOT_PASSWORD"),
        ) {
            (Ok(u), Ok(p)) => (u, p),
            _ => {
                if matches!(
                    cli.command,
                    Some(Commands::Config(_))
                        | Some(Commands::Context(_))
                        | Some(Commands::Completion(_))
                        | Some(Commands::Version)
                        | None
                ) {
                    // These commands don't require credentials
                    ("".to_string(), "".to_string())
                } else {
                    eprintln!("Error: No credentials found. Please set HROBOT_USERNAME and HROBOT_PASSWORD environment variables or create a context with 'hrobot context create'");
                    process::exit(1);
                }
            }
        }
    };

    // Create Robot client
    let robot = if !username.is_empty() {
        Some(hrobot::AsyncRobot::new_with_default_client(&username, &password))
    } else {
        None
    };

    // Execute command
    let result = match cli.command {
        Some(Commands::Server(args)) => {
            server::execute(robot.unwrap(), args, &cli.output).await
        }
        Some(Commands::Ip(args)) => ip::execute(robot.unwrap(), args, &cli.output).await,
        Some(Commands::Subnet(args)) => subnet::execute(robot.unwrap(), args, &cli.output).await,
        Some(Commands::SshKey(args)) => {
            ssh_key::execute(robot.unwrap(), args, &cli.output).await
        }
        Some(Commands::Firewall(args)) => {
            firewall::execute(robot.unwrap(), args, &cli.output).await
        }
        Some(Commands::Rdns(args)) => rdns::execute(robot.unwrap(), args, &cli.output).await,
        Some(Commands::Failover(args)) => {
            failover::execute(robot.unwrap(), args, &cli.output).await
        }
        Some(Commands::Reset(args)) => reset::execute(robot.unwrap(), args, &cli.output).await,
        Some(Commands::Boot(args)) => boot::execute(robot.unwrap(), args, &cli.output).await,
        Some(Commands::Storagebox(args)) => {
            storagebox::execute(robot.unwrap(), args, &cli.output).await
        }
        Some(Commands::Vswitch(args)) => {
            vswitch::execute(robot.unwrap(), args, &cli.output).await
        }
        Some(Commands::Config(args)) => config::execute(args, &config, &cli.config).await,
        Some(Commands::Context(args)) => context::execute(args, &config, &cli.config).await,
        Some(Commands::Completion(args)) => completion::execute(args).await,
        Some(Commands::Version) => {
            println!("hrobot {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        None => {
            eprintln!("No command specified. Use --help for usage information.");
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
