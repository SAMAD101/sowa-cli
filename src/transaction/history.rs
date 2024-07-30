use anyhow::Result;
use solana_sdk::signer::Signer;
use crate::utils::rpc::get_rpc_client;
use crate::wallet::load_keypair;

pub fn get_transaction_history() -> Result<Vec<String>> {
    let rpc_client = get_rpc_client()?;
    let keypair = load_keypair()?;
    let pubkey = keypair.pubkey();

    let signatures = rpc_client.get_signatures_for_address(&pubkey)?;
    Ok(signatures.into_iter().map(|sig_with_status| sig_with_status.signature).collect())
}