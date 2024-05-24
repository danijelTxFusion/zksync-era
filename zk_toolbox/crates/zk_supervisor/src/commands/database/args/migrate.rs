use super::DatabaseCommonArgs;
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

        DatabaseMigrateArgsFinal {
            prover: common.prover,
            core: common.core,
            chain: common.chain,
            yes: self.yes,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseMigrateArgsFinal {
    pub prover: bool,
    pub core: bool,
    pub chain: Option<String>,
    pub yes: bool,
}
