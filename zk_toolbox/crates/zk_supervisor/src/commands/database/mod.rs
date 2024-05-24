use self::args::{
    drop::DatabaseDropArgs, migrate::DatabaseMigrateArgs, new_migration::DatabaseNewMigrationArgs,
    setup::DatabaseSetupArgs,
};
use clap::Subcommand;
use xshell::Shell;

mod args;
mod drop;
mod migrate;
mod new_migration;
mod setup;

#[derive(Subcommand, Debug)]
pub enum DatabaseCommands {
    /// Drop databases
    Drop(DatabaseDropArgs),
    /// Migrate databases
    Migrate(DatabaseMigrateArgs),
    /// Create new migration
    NewMigration(DatabaseNewMigrationArgs),
    /// Setup databases
    Setup(DatabaseSetupArgs),
}

pub fn run(shell: &Shell, args: DatabaseCommands) -> anyhow::Result<()> {
    match args {
        DatabaseCommands::Drop(args) => drop::run(shell, args),
        DatabaseCommands::Migrate(args) => migrate::run(shell, args),
        DatabaseCommands::NewMigration(args) => new_migration::run(shell, args),
        DatabaseCommands::Setup(args) => setup::run(shell, args),
    }
}
