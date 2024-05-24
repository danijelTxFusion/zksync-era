use super::DatabaseCommonArgs;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct DatabaseCheckSqlxDataArgs {
    #[clap(flatten)]
    pub common: DatabaseCommonArgs,
}

impl DatabaseCheckSqlxDataArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseCheckSqlxDataArgsFinal {
        let common = self.common.fill_values_with_prompt("check sqlx data for");

        DatabaseCheckSqlxDataArgsFinal {
            prover: common.prover,
            core: common.core,
            chain: common.chain,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseCheckSqlxDataArgsFinal {
    pub prover: bool,
    pub core: bool,
    pub chain: Option<String>,
}
