use anyhow::anyhow;
use common::config::global_config;
use xshell::Shell;
use zk_inception::configs::{EcosystemConfig, Secrets};

const CORE_DAL_PATH: &str = "core/lib/dal";
const PROVER_DAL_PATH: &str = "prover/prover_dal";

#[derive(Debug, Clone)]
pub struct Dal {
    pub path: String,
    pub url: String,
}

pub fn get_prover_dal(shell: &Shell) -> anyhow::Result<Dal> {
    let secrets = get_secrets(shell)?;

    Ok(Dal {
        path: PROVER_DAL_PATH.to_string(),
        url: secrets.database.prover_url.clone(),
    })
}

pub fn get_core_dal(shell: &Shell) -> anyhow::Result<Dal> {
    let secrets = get_secrets(shell)?;

    Ok(Dal {
        path: CORE_DAL_PATH.to_string(),
        url: secrets.database.server_url.clone(),
    })
}

fn get_secrets(shell: &Shell) -> anyhow::Result<Secrets> {
    let ecosystem_config = EcosystemConfig::from_file(shell)?;
    let chain_config = ecosystem_config
        .load_chain(global_config().chain_name.clone())
        .ok_or(anyhow!("Chain not found"))?;
    let secrets = chain_config.get_secrets_config()?;

    Ok(secrets)
}
