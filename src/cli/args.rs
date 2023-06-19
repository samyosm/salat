use clap::Parser;

use super::commands::Commands;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}
