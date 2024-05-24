use super::{DatabaseCommonArgs, DatabaseCommonArgsFinal};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct DatabaseResetArgs {
    #[clap(flatten)]
    pub common: DatabaseCommonArgs,
}

impl DatabaseResetArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseResetArgsFinal {
        let common = self.common.fill_values_with_prompt("reset");

        DatabaseResetArgsFinal { common }
    }
}

#[derive(Debug)]
pub struct DatabaseResetArgsFinal {
    pub common: DatabaseCommonArgsFinal,
}
