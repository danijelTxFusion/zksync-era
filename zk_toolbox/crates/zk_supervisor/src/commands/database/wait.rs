use super::args::wait::DatabaseWaitArgs;
use crate::dals::{get_core_dal, get_prover_dal, Dal};
use common::{cmd::Cmd, logger, spinner::Spinner};
use xshell::{cmd, Shell};

pub fn run(shell: &Shell, args: DatabaseWaitArgs) -> anyhow::Result<()> {
    let args = args.fill_values_with_prompt();
    if !args.prover && !args.core {
        logger::outro("No databases selected");
        return Ok(());
    }

    logger::info("Waiting for databases to get ready for interaction");

    if args.prover {
        wait_database(
            shell,
            get_prover_dal(shell, args.chain.clone())?,
            args.tries,
        )?;
    }
    if args.core {
        wait_database(shell, get_core_dal(shell, args.chain)?, args.tries)?;
    }

    logger::outro("Databases ready for interaction");

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
