use super::args::drop::DatabaseDropArgs;
use crate::dals::{get_core_dal, get_prover_dal, Dal};
use common::{cmd::Cmd, logger, spinner::Spinner};
use xshell::{cmd, Shell};

pub fn run(shell: &Shell, args: DatabaseDropArgs) -> anyhow::Result<()> {
    let args = args.fill_values_with_prompt();
    if !args.common.prover && !args.common.core {
        logger::outro("No databases selected to drop");
        return Ok(());
    }

    logger::info("Dropping databases");

    if args.common.prover {
        drop_database(shell, get_prover_dal(shell)?)?;
    }
    if args.common.core {
        drop_database(shell, get_core_dal(shell)?)?;
    }

    logger::outro("Databases dropped successfully");

    Ok(())
}

pub fn drop_database(shell: &Shell, dal: Dal) -> anyhow::Result<()> {
    let spinner = Spinner::new(&format!("Dropping DB for dal {}...", dal.path));
    let url = dal.url;
    Cmd::new(cmd!(
        shell,
        "cargo sqlx database drop -y --database-url {url}"
    ))
    .run()?;
    spinner.finish();
    Ok(())
}
