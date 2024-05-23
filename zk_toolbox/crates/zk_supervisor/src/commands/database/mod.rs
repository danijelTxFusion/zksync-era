use self::args::{drop::DatabaseDropArgs, migrate::DatabaseMigrateArgs};
use clap::Subcommand;
use xshell::Shell;

mod args;
mod drop;
mod migrate;

#[derive(Subcommand, Debug)]
pub enum DatabaseCommands {
    /// Drop databases
    Drop(DatabaseDropArgs),
    /// Migrate databases
    Migrate(DatabaseMigrateArgs),
}

pub fn run(shell: &Shell, args: DatabaseCommands) -> anyhow::Result<()> {
    match args {
        DatabaseCommands::Drop(args) => drop::run(shell, args),
        DatabaseCommands::Migrate(args) => migrate::run(shell, args),
    }
}
