use super::{DatabaseCommonArgs, DatabaseCommonArgsFinal};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct DatabasePrepareArgs {
    #[clap(flatten)]
    pub common: DatabaseCommonArgs,
}

impl DatabasePrepareArgs {
    pub fn fill_values_with_prompt(self) -> DatabasePrepareArgsFinal {
        let common = self.common.fill_values_with_prompt("prepare");

        DatabasePrepareArgsFinal { common }
    }
}

#[derive(Debug)]
pub struct DatabasePrepareArgsFinal {
    pub common: DatabaseCommonArgsFinal,
}
