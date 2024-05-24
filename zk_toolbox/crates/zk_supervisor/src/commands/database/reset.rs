use super::{args::DatabaseCommonArgs, drop::drop_database, setup::setup_database};
use crate::dals::{get_core_dal, get_prover_dal, Dal};
use common::logger;
use std::path::Path;
use xshell::Shell;
use zk_inception::configs::EcosystemConfig;

pub async fn run(shell: &Shell, args: DatabaseCommonArgs) -> anyhow::Result<()> {
    let args = args.fill_values_with_prompt("reset");
    if !args.prover && !args.core {
        logger::outro("No databases selected");
        return Ok(());
    }

    let ecoseystem_config = EcosystemConfig::from_file(shell)?;

    if args.prover {
        logger::info("Resetting prover database");
        reset_database(
            shell,
            ecoseystem_config.link_to_code.clone(),
            get_prover_dal(shell)?,
        )
        .await?;
    }
    if args.core {
        logger::info("Resetting core database");
        reset_database(shell, ecoseystem_config.link_to_code, get_core_dal(shell)?).await?;
    }

    logger::outro("Databases resetted");

    Ok(())
}

async fn reset_database(
    shell: &Shell,
    link_to_code: impl AsRef<Path>,
    dal: Dal,
) -> anyhow::Result<()> {
    drop_database(dal.clone()).await?;
    setup_database(shell, link_to_code, dal)?;
    Ok(())
}
