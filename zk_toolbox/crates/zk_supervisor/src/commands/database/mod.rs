use self::args::{
    drop::DatabaseDropArgs, migrate::DatabaseMigrateArgs, new_migration::DatabaseNewMigrationArgs,
    reset::DatabaseResetArgs, setup::DatabaseSetupArgs, wait::DatabaseWaitArgs,
};
use clap::Subcommand;
use xshell::Shell;

mod args;
mod drop;
mod migrate;
mod new_migration;
mod reset;
mod setup;
mod wait;

#[derive(Subcommand, Debug)]
pub enum DatabaseCommands {
    /// Drop databases
    Drop(DatabaseDropArgs),
    /// Migrate databases
    Migrate(DatabaseMigrateArgs),
    /// Create new migration
    NewMigration(DatabaseNewMigrationArgs),
    /// Reset databases
    Reset(DatabaseResetArgs),
    /// Setup databases
    Setup(DatabaseSetupArgs),
    /// Wait for databases to be ready
    Wait(DatabaseWaitArgs),
}

pub fn run(shell: &Shell, args: DatabaseCommands) -> anyhow::Result<()> {
    match args {
        DatabaseCommands::Drop(args) => drop::run(shell, args),
        DatabaseCommands::Migrate(args) => migrate::run(shell, args),
        DatabaseCommands::NewMigration(args) => new_migration::run(shell, args),
        DatabaseCommands::Reset(args) => reset::run(shell, args),
        DatabaseCommands::Setup(args) => setup::run(shell, args),
        DatabaseCommands::Wait(args) => wait::run(shell, args),
    }
}
