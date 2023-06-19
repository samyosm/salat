use clap::Subcommand;
use enum_dispatch::enum_dispatch;

use super::{next::Next, notification::Notifs, setup::Setup, time::Time};

#[enum_dispatch]
#[derive(Subcommand, Debug)]
pub enum Commands {
    Time,
    Setup,
    #[command(subcommand)]
    Notifs,
    #[command(subcommand)]
    Next,
}

#[enum_dispatch(Commands)]
pub trait Execute {
    fn execute(&self);
}
