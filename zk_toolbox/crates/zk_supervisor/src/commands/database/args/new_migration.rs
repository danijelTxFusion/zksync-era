use clap::{Parser, ValueEnum};
use common::{Prompt, PromptSelect};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Parser)]
pub struct DatabaseNewMigrationArgs {
    /// Database to create new migration for
    #[clap(long)]
    pub database: Option<SelectedDatabase>,
    /// Migration name
    #[clap(long)]
    pub name: Option<String>,
    /// Selected chain, if not provided default will be used
    #[clap(long)]
    pub chain: Option<String>,
    /// Skip confirmation
    #[clap(short)]
    pub yes: bool,
}

impl DatabaseNewMigrationArgs {
    pub fn fill_values_with_prompt(self) -> DatabaseNewMigrationArgsFinal {
        let selected_database = self.database.unwrap_or_else(|| {
            PromptSelect::new(
                "What database do you want to create a new migration for?",
                SelectedDatabase::iter(),
            )
            .ask()
        });
        let name = self
            .name
            .unwrap_or_else(|| Prompt::new("How do you want to name the migration?").ask());

        DatabaseNewMigrationArgsFinal {
            selected_database,
            name,
            chain: self.chain,
            yes: self.yes,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseNewMigrationArgsFinal {
    pub selected_database: SelectedDatabase,
    pub name: String,
    pub chain: Option<String>,
    pub yes: bool,
}

#[derive(Debug, Clone, ValueEnum, EnumIter, PartialEq, Eq, Display)]
pub enum SelectedDatabase {
    Prover,
    Core,
}
