use super::DatabaseCommonArgs;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct DatabaseWaitArgs {
    #[clap(flatten)]
    pub common: DatabaseCommonArgs,
    /// Number of tries to wait for the database to be ready
    #[clap(long, default_value = "5")]
    pub tries: u32,
}

impl DatabaseWaitArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseWaitArgsFinal {
        let common = self.common.fill_values_with_prompt("wait for");

        DatabaseWaitArgsFinal {
            prover: common.prover,
            core: common.core,
            chain: common.chain,
            tries: self.tries,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseWaitArgsFinal {
    pub prover: bool,
    pub core: bool,
    pub chain: Option<String>,
    pub tries: u32,
}
