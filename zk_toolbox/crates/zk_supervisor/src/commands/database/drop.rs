use super::args::drop::DatabaseDropArgs;
use anyhow::anyhow;
use common::{cmd::Cmd, logger, spinner::Spinner, PromptConfirm};
use xshell::{cmd, Shell};
use zk_inception::configs::EcosystemConfig;

pub fn run(shell: &Shell, args: DatabaseDropArgs) -> anyhow::Result<()> {
    let args = args.fill_values_with_prompt();
    if !args.prover && !args.core {
        logger::outro("No databases selected to drop");
        return Ok(());
    }

    if !args.yes
        && !PromptConfirm::new("Are you sure you want to drop the selected databases?").ask()
    {
        logger::outro("Databases not dropped");
        return Ok(());
    }

    logger::info("Dropping databases");

    let ecosystem_config = EcosystemConfig::from_file(shell)?;
    let chain_config = ecosystem_config
        .load_chain(args.chain.clone())
        .ok_or(anyhow!("Chain not found"))?;
    let secrets = chain_config.get_secrets_config()?;

    if args.prover {
        drop_database(shell, "prover", &secrets.database.prover_url)?;
    }
    if args.core {
        drop_database(shell, "core", &secrets.database.server_url)?;
    }

    logger::outro("Databases dropped successfully");

    Ok(())
}

fn drop_database(shell: &Shell, db_name: &str, db_url: &str) -> anyhow::Result<()> {
    let spinner = Spinner::new(&format!("Dropping {db_name} database"));
    Cmd::new(cmd!(
        shell,
        "cargo sqlx database drop -y --database-url {db_url}"
    ))
    .run()?;
    spinner.finish();
    Ok(())
}
