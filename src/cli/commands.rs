use anyhow::Result;
use clap::{Parser, Subcommand};
use solana_sdk::{
    pubkey::Pubkey,
    signer::Signer,
};
use crate::wallet::{create_account, get_balance};
use crate::transaction::{send_transaction, get_transaction_history};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateAccount,
    Balance { address: Option<Pubkey> },
    Send { to: Pubkey, amount: f64 },
    History,
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
            let balance = get_balance(address.as_ref())?;
        let address_str = address.map_or_else(
            || "your account".to_string(),
            |pubkey| pubkey.to_string()
        );
        println!("Balance of {}: {} SOL", address_str, balance);
        Ok(())
        }
        Commands::Send { to, amount } => {
            let signature = send_transaction(to, *amount)?;
            println!("Transaction sent: {}", signature);
            Ok(())
        }
        Commands::History => {
            let history = get_transaction_history()?;
            for (index, signature) in history.iter().enumerate() {
                println!("{}. {}", index + 1, signature);
            }
            Ok(())
        }
    }
}