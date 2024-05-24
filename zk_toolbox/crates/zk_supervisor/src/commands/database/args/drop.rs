use super::{DatabaseCommonArgs, DatabaseCommonArgsFinal};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct DatabaseDropArgs {
    #[clap(flatten)]
    pub common: DatabaseCommonArgs,
}

impl DatabaseDropArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseDropArgsFinal {
        let common = self.common.fill_values_with_prompt("drop");

        DatabaseDropArgsFinal { common }
    }
}

#[derive(Debug)]
pub struct DatabaseDropArgsFinal {
    pub common: DatabaseCommonArgsFinal,
}
