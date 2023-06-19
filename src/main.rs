mod cli;

use clap::Parser;
use cli::args::CliArgs;

use crate::cli::commands::Execute;

fn main() {
    let args = CliArgs::parse();

    if let Some(command) = &args.command {
        command.execute();
    } else {
        todo!("TUI")
    }
}
