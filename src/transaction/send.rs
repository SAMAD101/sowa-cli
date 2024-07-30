use anyhow::Result;
use solana_sdk::{
    pubkey::Pubkey,
    transaction::Transaction,
    signer::Signer,
};
use crate::utils::rpc::get_rpc_client;
use crate::wallet::load_keypair;

pub fn send_transaction(to: &Pubkey, amount: f64) -> Result<String> {
    let rpc_client = get_rpc_client()?;
    let keypair = load_keypair()?;
    let from = keypair.pubkey();

    let lamports = (amount * 1e9) as u64;
    let recent_blockhash = rpc_client.get_latest_blockhash()?;

    let transaction = Transaction::new_signed_with_payer(
        &[solana_sdk::system_instruction::transfer(
            &from,
            to,
            lamports,
        )],
        Some(&from),
        &[&keypair],
        recent_blockhash,
    );

    let signature = rpc_client.send_and_confirm_transaction(&transaction)?;
    Ok(signature.to_string())
}