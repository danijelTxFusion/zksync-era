use crate::{envy_load, FromEnv};
use zksync_config::configs::native_erc20_fetcher::NativeErc20FetcherConfig;

impl FromEnv for NativeErc20FetcherConfig {
    fn from_env() -> anyhow::Result<Self> {
        envy_load("native_erc20_fetcher", "NATIVE_TOKEN_FETCHER_")
    }
}
