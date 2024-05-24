use super::{args::reset::DatabaseResetArgs, drop::drop_database, setup::setup_database};
use crate::dals::{get_core_dal, get_prover_dal, Dal};
use common::{cmd::Cmd, logger, spinner::Spinner};
use std::path::Path;
use xshell::{cmd, Shell};
use zk_inception::configs::EcosystemConfig;

pub fn run(shell: &Shell, args: DatabaseResetArgs) -> anyhow::Result<()> {
    let args = args.fill_values_with_prompt();
    if !args.common.prover && !args.common.core {
        logger::outro("No databases selected");
        return Ok(());
    }

    let ecoseystem_config = EcosystemConfig::from_file(shell)?;

    if args.common.prover {
        logger::info("Resetting prover database");
        reset_database(
            shell,
            ecoseystem_config.link_to_code.clone(),
            get_prover_dal(shell)?,
        )?;
    }
    if args.common.core {
        logger::info("Resetting core database");
        reset_database(shell, ecoseystem_config.link_to_code, get_core_dal(shell)?)?;
    }

    logger::outro("Databases resetted");

    Ok(())
}

fn reset_database(shell: &Shell, link_to_code: impl AsRef<Path>, dal: Dal) -> anyhow::Result<()> {
    wait_database(shell, dal.clone(), 5)?;
    drop_database(shell, dal.clone())?;
    setup_database(shell, link_to_code, dal)?;
    Ok(())
}

fn wait_database(shell: &Shell, dal: Dal, tries: u32) -> anyhow::Result<()> {
    let url = dal.url;
    let spinner = Spinner::new(&format!(
        "Waiting until DB for dal {} is ready...",
        dal.path
    ));

    for i in 0..tries {
        let output = Cmd::new(cmd!(shell, "pg_isready -d {url}")).run_with_output()?;
        if output.status.success() {
            spinner.finish();
            return Ok(());
        }

        // Only sleep if there are more tries left
        if i < tries - 1 {
            std::thread::sleep(std::time::Duration::from_secs(1))
        }
    }

    // If we reach here, it means the database is not ready
    spinner.fail();
    anyhow::bail!(
        "DB for dal {} is not ready after {} attempts",
        dal.path,
        tries
    );
}
