use clap::Parser;
use common::PromptConfirm;

#[derive(Debug, Parser)]
pub struct DatabaseMigrateArgs {
    /// Prover
    #[clap(short, long, default_missing_value = "true", num_args = 0..=1)]
    pub prover: Option<bool>,
    /// Core
    #[clap(short, long, default_missing_value = "true", num_args = 0..=1)]
    pub core: Option<bool>,
    /// Selected chain, if not provided default will be used
    #[clap(long)]
    pub chain: Option<String>,
    /// Skip confirmation
    #[clap(short)]
    pub yes: bool,
}

impl DatabaseMigrateArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseMigrateArgsFinal {
        let prover = self.prover.unwrap_or_else(|| {
            PromptConfirm::new("Do you want to migrate the prover database?").ask()
        });

        let core = self
            .core
            .unwrap_or_else(|| PromptConfirm::new("Do you want to migrate the core database?").ask());

        DatabaseMigrateArgsFinal {
            prover,
            core,
            chain: self.chain,
            yes: self.yes,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseMigrateArgsFinal {
    pub prover: bool,
    pub core: bool,
    pub chain: Option<String>,
    pub yes: bool,
}
