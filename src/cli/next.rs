use clap::Subcommand;

#[derive(Subcommand, Debug)]
/// Information about the next due prayer
pub enum NextCommands {
    /// Prayer name
    Name,
    /// Due time
    Time,
    /// Time left
    Left,
}
