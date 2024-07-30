use anyhow::Result;
use solana_sdk::{signature::Keypair, pubkey::Pubkey};
use crate::utils::rpc::get_rpc_client;
use super::keypair::save_keypair;

pub fn create_account() -> Result<Keypair> {
    let keypair = Keypair::new();
    save_keypair(&keypair)?;
    Ok(keypair)
}

pub fn get_balance(address: &Pubkey) -> Result<f64> {
    let rpc_client = get_rpc_client()?;
    let balance = rpc_client.get_balance(address)?;
    Ok(balance as f64 / 1e9) // Convert lamports to SOL
}