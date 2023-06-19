mod cli;

use clap::Parser;
use cli::args::CliArgs;

fn main() {
    let args = CliArgs::parse();

    println!("{:#?}", args);
}
