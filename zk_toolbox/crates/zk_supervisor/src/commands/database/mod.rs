use self::args::{
    check_sqlx_data::DatabaseCheckSqlxDataArgs, drop::DatabaseDropArgs,
    migrate::DatabaseMigrateArgs, new_migration::DatabaseNewMigrationArgs,
    prepare::DatabasePrepareArgs, reset::DatabaseResetArgs, setup::DatabaseSetupArgs,
    wait::DatabaseWaitArgs,
};
use clap::Subcommand;
use xshell::Shell;

mod args;
mod check_sqlx_data;
mod drop;
mod migrate;
mod new_migration;
mod prepare;
mod reset;
mod setup;
mod wait;

#[derive(Subcommand, Debug)]
pub enum DatabaseCommands {
    /// Check sqlx-data.json is up to date
    CheckSqlxData(DatabaseCheckSqlxDataArgs),
    /// Drop databases
    Drop(DatabaseDropArgs),
    /// Migrate databases
    Migrate(DatabaseMigrateArgs),
    /// Create new migration
    NewMigration(DatabaseNewMigrationArgs),
    /// Prepare sqlx-data.json
    Prepare(DatabasePrepareArgs),
    /// Reset databases
    Reset(DatabaseResetArgs),
    /// Setup databases
    Setup(DatabaseSetupArgs),
    /// Wait for databases to be ready
    Wait(DatabaseWaitArgs),
}

pub fn run(shell: &Shell, args: DatabaseCommands) -> anyhow::Result<()> {
    match args {
        DatabaseCommands::CheckSqlxData(args) => check_sqlx_data::run(shell, args),
        DatabaseCommands::Drop(args) => drop::run(shell, args),
        DatabaseCommands::Migrate(args) => migrate::run(shell, args),
        DatabaseCommands::NewMigration(args) => new_migration::run(shell, args),
        DatabaseCommands::Prepare(args) => prepare::run(shell, args),
        DatabaseCommands::Reset(args) => reset::run(shell, args),
        DatabaseCommands::Setup(args) => setup::run(shell, args),
        DatabaseCommands::Wait(args) => wait::run(shell, args),
    }
}
