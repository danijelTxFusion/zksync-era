use self::args::drop::DatabaseDropArgs;
use clap::Subcommand;
use xshell::Shell;

mod args;
mod drop;

#[derive(Subcommand, Debug)]
pub enum DatabaseCommands {
    /// Drop databases
    Drop(DatabaseDropArgs),
}

pub(crate) fn run(shell: &Shell, args: DatabaseCommands) -> anyhow::Result<()> {
    match args {
        DatabaseCommands::Drop(args) => drop::run(shell, args),
    }
}
