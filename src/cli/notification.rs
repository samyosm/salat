use clap::Subcommand;

#[derive(Subcommand, Debug)]
/// Manage the notification daemon
pub enum NotificationCommands {
    Stop,
    Start,
    Restart,
}
