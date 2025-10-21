use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};

#[derive(Args)]
pub struct CompletionArgs {
    /// Shell to generate completion for
    #[arg(value_enum)]
    shell: Shell,
}

pub async fn execute(args: CompletionArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = crate::Cli::command();
    let bin_name = cmd.get_name().to_string();
    generate(args.shell, &mut cmd, bin_name, &mut std::io::stdout());
    Ok(())
}
