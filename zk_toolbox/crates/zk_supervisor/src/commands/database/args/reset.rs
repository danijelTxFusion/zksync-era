use clap::Parser;
use common::PromptConfirm;

#[derive(Debug, Parser)]
pub struct DatabaseResetArgs {
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

impl DatabaseResetArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseResetArgsFinal {
        let prover = self.prover.unwrap_or_else(|| {
            PromptConfirm::new("Do you want to reset the prover database?").ask()
        });

        let core = self
            .core
            .unwrap_or_else(|| PromptConfirm::new("Do you want to reset the core database?").ask());

        DatabaseResetArgsFinal {
            prover,
            core,
            chain: self.chain,
            yes: self.yes,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseResetArgsFinal {
    pub prover: bool,
    pub core: bool,
    pub chain: Option<String>,
    pub yes: bool,
}
