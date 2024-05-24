use super::DatabaseCommonArgs;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct DatabasePrepareArgs {
    #[clap(flatten)]
    pub common: DatabaseCommonArgs,
}

impl DatabasePrepareArgs {
    pub fn fill_values_with_prompt(self) -> DatabasePrepareArgsFinal {
        let common = self.common.fill_values_with_prompt("prepare");

        DatabasePrepareArgsFinal {
            prover: common.prover,
            core: common.core,
            chain: common.chain,
        }
    }
}

#[derive(Debug)]
pub struct DatabasePrepareArgsFinal {
    pub prover: bool,
    pub core: bool,
    pub chain: Option<String>,
}
