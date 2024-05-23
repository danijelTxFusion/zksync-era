use anyhow::anyhow;
use xshell::Shell;
use zk_inception::configs::{EcosystemConfig, Secrets};

const CORE_DAL_PATH: &str = "core/lib/dal";
const PROVER_DAL_PATH: &str = "prover/prover_dal";

pub struct Dal {
    pub path: String,
    pub url: String,
}

pub fn get_prover_dal(shell: &Shell, chain: Option<String>) -> anyhow::Result<Dal> {
    let secrets = get_secrets(shell, chain)?;

    Ok(Dal {
        path: PROVER_DAL_PATH.to_string(),
        url: secrets.database.prover_url.clone(),
    })
}

pub fn get_core_dal(shell: &Shell, chain: Option<String>) -> anyhow::Result<Dal> {
    let secrets = get_secrets(shell, chain)?;

    Ok(Dal {
        path: CORE_DAL_PATH.to_string(),
        url: secrets.database.server_url.clone(),
    })
}

fn get_secrets(shell: &Shell, chain: Option<String>) -> anyhow::Result<Secrets> {
    let ecosystem_config = EcosystemConfig::from_file(shell)?;
    let chain_config = ecosystem_config
        .load_chain(chain)
        .ok_or(anyhow!("Chain not found"))?;
    let secrets = chain_config.get_secrets_config()?;

    Ok(secrets)
}
