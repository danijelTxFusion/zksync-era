use clap::Parser;
use common::PromptConfirm;

#[derive(Debug, Parser)]
pub struct DatabaseCheckSqlxDataArgs {
    /// Prover
    #[clap(short, long, default_missing_value = "true", num_args = 0..=1)]
    pub prover: Option<bool>,
    /// Core
    #[clap(short, long, default_missing_value = "true", num_args = 0..=1)]
    pub core: Option<bool>,
    /// Selected chain, if not provided default will be used
    #[clap(long)]
    pub chain: Option<String>,
}

impl DatabaseCheckSqlxDataArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseCheckSqlxDataArgsFinal {
        let prover = self.prover.unwrap_or_else(|| {
            PromptConfirm::new("Do you want to check sqlx data for the prover database?").ask()
        });

        let core = self.core.unwrap_or_else(|| {
            PromptConfirm::new("Do you want to check sqlx data for the core database?").ask()
        });

        DatabaseCheckSqlxDataArgsFinal {
            prover,
            core,
            chain: self.chain,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseCheckSqlxDataArgsFinal {
    pub prover: bool,
    pub core: bool,
    pub chain: Option<String>,
}
