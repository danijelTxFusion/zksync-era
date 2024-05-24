use clap::Parser;
use common::PromptConfirm;

#[derive(Debug, Parser)]
pub struct DatabaseWaitArgs {
    /// Prover
    #[clap(short, long, default_missing_value = "true", num_args = 0..=1)]
    pub prover: Option<bool>,
    /// Core
    #[clap(short, long, default_missing_value = "true", num_args = 0..=1)]
    pub core: Option<bool>,
    /// Selected chain, if not provided default will be used
    #[clap(long)]
    pub chain: Option<String>,
    /// Number of tries to wait for the database to be ready
    #[clap(long, default_value = "5")]
    pub tries: u32,
}

impl DatabaseWaitArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseWaitArgsFinal {
        let prover = self.prover.unwrap_or_else(|| {
            PromptConfirm::new("Do you want to wait for the prover database?").ask()
        });

        let core = self.core.unwrap_or_else(|| {
            PromptConfirm::new("Do you want to wait for the core database?").ask()
        });

        DatabaseWaitArgsFinal {
            prover,
            core,
            chain: self.chain,
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
