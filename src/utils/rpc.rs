use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use crate::utils::config::get_config;

pub fn get_rpc_client() -> Result<RpcClient> {
    let config = get_config()?;
    Ok(RpcClient::new(config.rpc_url))
}