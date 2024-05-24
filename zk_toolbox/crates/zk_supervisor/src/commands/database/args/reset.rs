use super::DatabaseCommonArgs;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct DatabaseResetArgs {
    #[clap(flatten)]
    pub common: DatabaseCommonArgs,
    /// Skip confirmation
    #[clap(short)]
    pub yes: bool,
}

impl DatabaseResetArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseResetArgsFinal {
        let common = self.common.fill_values_with_prompt("reset");

        DatabaseResetArgsFinal {
            prover: common.prover,
            core: common.core,
            chain: common.chain,
            yes: self.yes,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseResetArgsFinal {
    pub prover: bool,
    pub core: bool,
    pub chain: Option<String>,
    pub yes: bool,
}
