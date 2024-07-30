use anyhow::Result;
use solana_sdk::{
    signature::Keypair,
    pubkey::Pubkey,
    native_token::LAMPORTS_PER_SOL,
    signer::Signer,
};
use crate::utils::rpc::get_rpc_client;
use crate::wallet::load_keypair;
use super::keypair::save_keypair;

pub fn create_account() -> Result<Keypair> {
    let keypair = Keypair::new();
    save_keypair(&keypair)?;
    Ok(keypair)
}

pub fn get_balance(address: Option<&Pubkey>) -> Result<f64> {
    let rpc_client = get_rpc_client()?;

    let balance = match address {
        Some(addr) => rpc_client.get_balance(addr)?,
        None => {
            let keypair = load_keypair()?;
            rpc_client.get_balance(&keypair.pubkey())?
        }
    };

    Ok(balance as f64 / LAMPORTS_PER_SOL as f64)
}