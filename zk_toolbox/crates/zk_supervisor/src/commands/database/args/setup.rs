use super::{DatabaseCommonArgs, DatabaseCommonArgsFinal};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct DatabaseSetupArgs {
    #[clap(flatten)]
    pub common: DatabaseCommonArgs,
}

impl DatabaseSetupArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseSetupArgsFinal {
        let common = self.common.fill_values_with_prompt("setup");

        DatabaseSetupArgsFinal { common }
    }
}

#[derive(Debug)]
pub struct DatabaseSetupArgsFinal {
    pub common: DatabaseCommonArgsFinal,
}
