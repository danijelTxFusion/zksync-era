use super::args::new_migration::{DatabaseNewMigrationArgs, SelectedDatabase};
use crate::dals::{get_core_dal, get_prover_dal, Dal};
use common::{cmd::Cmd, spinner::Spinner};
use std::path::Path;
use xshell::{cmd, Shell};
use zk_inception::configs::EcosystemConfig;

pub fn run(shell: &Shell, args: DatabaseNewMigrationArgs) -> anyhow::Result<()> {
    let args = args.fill_values_with_prompt();

    let dal = match args.selected_database {
        SelectedDatabase::Core => get_core_dal(shell, args.chain.clone())?,
        SelectedDatabase::Prover => get_prover_dal(shell, args.chain)?,
    };
    let ecosystem_config = EcosystemConfig::from_file(shell)?;

    generate_migration(shell, &ecosystem_config.link_to_code, dal, args.name)?;

    Ok(())
}

fn generate_migration(
    shell: &Shell,
    link_to_code: impl AsRef<Path>,
    dal: Dal,
    name: String,
) -> anyhow::Result<()> {
    let dir = link_to_code.as_ref().join(&dal.path);
    let _dir_guard = shell.push_dir(dir);

    let spinner = Spinner::new(&format!("Creating new DB migration for dal {}", dal.path));
    Cmd::new(cmd!(shell, "cargo sqlx migrate add -r {name}")).run()?;
    spinner.finish();

    Ok(())
}
