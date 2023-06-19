use clap::Args;

use super::commands::Execute;

#[derive(Debug, Args)]
/// Displays information about a prayer
pub struct Time {
    /// The prayer name: fajr, dhuhr, asr, maghrib, isha
    name: String,

    /// Displays the time left before a prayer
    #[arg(short, long)]
    left: bool,
}

impl Execute for Time {
    fn execute(&self) {
        todo!()
    }
}
