use clap::{Parser, Subcommand};
use solana_sdk::{
    pubkey::Pubkey,
    signer::Signer,
};
use crate::wallet::{create_account, get_balance};
use crate::transaction::send_transaction;
use anyhow::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateAccount,
    Balance { address: Pubkey },
    Send { to: Pubkey, amount: f64 },
}

pub fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::CreateAccount => {
            let account = create_account()?;
            println!("Created new account: {}", account.pubkey());
            Ok(())
        }
        Commands::Balance { address } => {
            let balance = get_balance(address)?;
            println!("Balance of {}: {} SOL", address, balance);
            Ok(())
        }
        Commands::Send { to, amount } => {
            let signature = send_transaction(to, *amount)?;
            println!("Transaction sent: {}", signature);
            Ok(())
        }
    }
}