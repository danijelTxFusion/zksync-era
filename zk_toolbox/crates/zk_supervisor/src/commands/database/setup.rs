use super::args::setup::DatabaseSetupArgs;
use crate::dals::{get_core_dal, get_prover_dal, Dal};
use common::{cmd::Cmd, logger, spinner::Spinner};
use std::path::Path;
use xshell::{cmd, Shell};
use zk_inception::configs::EcosystemConfig;

pub fn run(shell: &Shell, args: DatabaseSetupArgs) -> anyhow::Result<()> {
    let args = args.fill_values_with_prompt();
    if !args.prover && !args.core {
        logger::outro("No databases selected to setup");
        return Ok(());
    }

    let ecosystem_config = EcosystemConfig::from_file(shell)?;

    logger::info("Setting up databases");

    if args.prover {
        setup_database(
            shell,
            &ecosystem_config.link_to_code,
            get_prover_dal(shell, args.chain.clone())?,
        )?;
    }
    if args.core {
        setup_database(
            shell,
            &ecosystem_config.link_to_code,
            get_core_dal(shell, args.chain)?,
        )?;
    }

    logger::outro("Databases set up successfully");

    Ok(())
}

pub fn setup_database(
    shell: &Shell,
    link_to_code: impl AsRef<Path>,
    dal: Dal,
) -> anyhow::Result<()> {
    let dir = link_to_code.as_ref().join(&dal.path);
    let _dir_guard = shell.push_dir(dir);
    let url = dal.url;

    let spinner = Spinner::new(&format!("Setting up DB for dal {}...", dal.path));
    Cmd::new(cmd!(
        shell,
        "cargo sqlx database create --database-url {url}"
    ))
    .run()?;
    Cmd::new(cmd!(shell, "cargo sqlx migrate run --database-url {url}")).run()?;
    spinner.finish();
    Ok(())
}
