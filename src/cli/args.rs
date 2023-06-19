use clap::{Parser, Subcommand};

use super::{next::NextCommands, notification::NotificationCommands, setup::Setup, time::Time};

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Time(Time),
    Setup(Setup),
    #[command(subcommand)]
    Notification(NotificationCommands),
    #[command(subcommand)]
    Next(NextCommands),
}
