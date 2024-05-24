use super::args::DatabaseCommonArgs;
use crate::dals::{get_core_dal, get_prover_dal, Dal};
use common::{
    db::{drop_db_if_exists, split_db_url},
    logger,
    spinner::Spinner,
};
use xshell::Shell;

pub async fn run(shell: &Shell, args: DatabaseCommonArgs) -> anyhow::Result<()> {
    let args = args.fill_values_with_prompt("drop");
    if !args.prover && !args.core {
        logger::outro("No databases selected to drop");
        return Ok(());
    }

    logger::info("Dropping databases");

    if args.prover {
        drop_database(get_prover_dal(shell)?).await?;
    }
    if args.core {
        drop_database(get_core_dal(shell)?).await?;
    }

    logger::outro("Databases dropped successfully");

    Ok(())
}

pub async fn drop_database(dal: Dal) -> anyhow::Result<()> {
    let spinner = Spinner::new(&format!("Dropping DB for dal {}...", dal.path));
    let (url, db_name) = split_db_url(&dal.url);
    drop_db_if_exists(&url, &db_name).await?;
    spinner.finish();
    Ok(())
}
