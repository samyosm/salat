use clap::Args;

use super::commands::Execute;

#[derive(Debug, Args)]
/// Attempts to automatically set longitude and latitude in config
pub struct Setup {}

impl Execute for Setup {
    fn execute(&self) {
        todo!()
    }
}
