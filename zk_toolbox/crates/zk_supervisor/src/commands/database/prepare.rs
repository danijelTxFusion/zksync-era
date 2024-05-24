use super::args::prepare::DatabasePrepareArgs;
use crate::dals::{get_core_dal, get_prover_dal, Dal};
use common::{cmd::Cmd, logger, spinner::Spinner};
use std::path::Path;
use xshell::{cmd, Shell};
use zk_inception::configs::EcosystemConfig;

pub fn run(shell: &Shell, args: DatabasePrepareArgs) -> anyhow::Result<()> {
    let args = args.fill_values_with_prompt();
    if !args.common.prover && !args.common.core {
        logger::outro("No databases selected to prepare");
        return Ok(());
    }

    let ecosystem_config = EcosystemConfig::from_file(shell)?;

    logger::info("Preparing sqlx data");

    if args.common.prover {
        prepare_sqlx_data(
            shell,
            &ecosystem_config.link_to_code,
            get_prover_dal(shell)?,
        )?;
    }
    if args.common.core {
        prepare_sqlx_data(shell, &ecosystem_config.link_to_code, get_core_dal(shell)?)?;
    }

    logger::outro("Databases sqlx data prepared successfully");

    Ok(())
}

pub fn prepare_sqlx_data(
    shell: &Shell,
    link_to_code: impl AsRef<Path>,
    dal: Dal,
) -> anyhow::Result<()> {
    let dir = link_to_code.as_ref().join(&dal.path);
    let _dir_guard = shell.push_dir(dir);
    let url = dal.url;

    let spinner = Spinner::new(&format!("Preparing sqlx data for dal {}...", dal.path));
    Cmd::new(cmd!(
        shell,
        "cargo sqlx prepare --database-url {url} -- --tests"
    ))
    .run()?;
    spinner.finish();
    Ok(())
}
