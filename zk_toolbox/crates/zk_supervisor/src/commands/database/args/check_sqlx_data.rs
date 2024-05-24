use super::{DatabaseCommonArgs, DatabaseCommonArgsFinal};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct DatabaseCheckSqlxDataArgs {
    #[clap(flatten)]
    pub common: DatabaseCommonArgs,
}

impl DatabaseCheckSqlxDataArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseCheckSqlxDataArgsFinal {
        let common = self.common.fill_values_with_prompt("check sqlx data for");

        DatabaseCheckSqlxDataArgsFinal { common }
    }
}

#[derive(Debug)]
pub struct DatabaseCheckSqlxDataArgsFinal {
    pub common: DatabaseCommonArgsFinal,
}
