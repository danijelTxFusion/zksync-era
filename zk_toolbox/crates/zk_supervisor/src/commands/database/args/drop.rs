use super::DatabaseCommonArgs;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct DatabaseDropArgs {
    #[clap(flatten)]
    pub common: DatabaseCommonArgs,
    /// Skip confirmation
    #[clap(short)]
    pub yes: bool,
}

impl DatabaseDropArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseDropArgsFinal {
        let common = self.common.fill_values_with_prompt("drop");

        DatabaseDropArgsFinal {
            prover: common.prover,
            core: common.core,
            chain: common.chain,
            yes: self.yes,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseDropArgsFinal {
    pub prover: bool,
    pub core: bool,
    pub chain: Option<String>,
    pub yes: bool,
}
