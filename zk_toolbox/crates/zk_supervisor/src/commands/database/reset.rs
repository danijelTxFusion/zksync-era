use super::{
    args::reset::DatabaseResetArgs, drop::drop_database, setup::setup_database, wait::wait_database,
};
use crate::dals::{get_core_dal, get_prover_dal, Dal};
use common::{logger, PromptConfirm};
use std::path::Path;
use xshell::Shell;
use zk_inception::configs::EcosystemConfig;

pub fn run(shell: &Shell, args: DatabaseResetArgs) -> anyhow::Result<()> {
    let args = args.fill_values_with_prompt();
    if !args.prover && !args.core {
        logger::outro("No databases selected");
        return Ok(());
    }

    if !args.yes
        && !PromptConfirm::new("Are you sure you want to reset the selected databases?").ask()
    {
        logger::outro("Databases not resetted");
        return Ok(());
    }

    let ecoseystem_config = EcosystemConfig::from_file(shell)?;

    if args.prover {
        logger::info("Resetting prover database");
        reset_database(
            shell,
            ecoseystem_config.link_to_code.clone(),
            get_prover_dal(shell, args.chain.clone())?,
        )?;
    }
    if args.core {
        logger::info("Resetting core database");
        reset_database(
            shell,
            ecoseystem_config.link_to_code,
            get_core_dal(shell, args.chain)?,
        )?;
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
