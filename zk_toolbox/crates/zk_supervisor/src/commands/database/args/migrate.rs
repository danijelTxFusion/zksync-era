use super::{DatabaseCommonArgs, DatabaseCommonArgsFinal};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct DatabaseMigrateArgs {
    #[clap(flatten)]
    pub common: DatabaseCommonArgs,
    /// Skip confirmation
    #[clap(short)]
    pub yes: bool,
}

impl DatabaseMigrateArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseMigrateArgsFinal {
        let common = self.common.fill_values_with_prompt("migrate");

        DatabaseMigrateArgsFinal { common }
    }
}

#[derive(Debug)]
pub struct DatabaseMigrateArgsFinal {
    pub common: DatabaseCommonArgsFinal,
}
