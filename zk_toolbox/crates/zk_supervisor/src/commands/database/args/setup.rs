use super::DatabaseCommonArgs;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct DatabaseSetupArgs {
    #[clap(flatten)]
    pub common: DatabaseCommonArgs,
}

impl DatabaseSetupArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseSetupArgsFinal {
        let common = self.common.fill_values_with_prompt("setup");

        DatabaseSetupArgsFinal {
            prover: common.prover,
            core: common.core,
            chain: common.chain,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseSetupArgsFinal {
    pub prover: bool,
    pub core: bool,
    pub chain: Option<String>,
}
