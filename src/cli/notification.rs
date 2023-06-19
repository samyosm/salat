use clap::Subcommand;

use super::commands::Execute;

#[derive(Subcommand, Debug)]
/// Manage the notification daemon
pub enum Notifs {
    Stop,
    Start,
    Restart,
}

impl Execute for Notifs {
    fn execute(&self) {
        todo!()
    }
}
