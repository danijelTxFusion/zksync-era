use clap::Parser;

pub mod check_sqlx_data;
pub mod drop;
pub mod migrate;
pub mod new_migration;
pub mod prepare;
pub mod reset;
pub mod setup;
pub mod wait;

#[derive(Debug, Parser)]
pub struct DatabaseCommonArgs {
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

impl DatabaseCommonArgs {
    pub fn fill_values_with_prompt(self, verb: &str) -> DatabaseCommonArgsFinal {
        let prover = self.prover.unwrap_or_else(|| {
            common::PromptConfirm::new(&format!("Do you want to {verb} the prover database?")).ask()
        });

        let core = self.core.unwrap_or_else(|| {
            common::PromptConfirm::new(&format!("Do you want to {verb} the core database?")).ask()
        });

        DatabaseCommonArgsFinal {
            prover,
            core,
            chain: self.chain,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseCommonArgsFinal {
    pub prover: bool,
    pub core: bool,
    pub chain: Option<String>,
}
