use clap::Subcommand;

use super::commands::Execute;

#[derive(Subcommand, Debug)]
/// Information about the next due prayer
pub enum Next {
    /// Prayer name
    Name,
    /// Due time
    Time,
    /// Time left
    Left,
}

impl Execute for Next {
    fn execute(&self) {
        todo!()
    }
}
