# Sowa - Solana Wallet CLI

Sowa (SOlana-WAllet) is a command-line interface tool for managing Solana wallets and performing basic operations on the Solana blockchain.

## Features

- Create new Solana accounts
- Check account balances
- Send SOL transactions
- View transaction history
- Configurable RPC endpoint

## Installation

### Prerequisites

- Rust and Cargo
- Git

### Building from source

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/sowa.git
   cd sowa
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. The binary will be available at `target/release/sowa`

## Usage

You can run Sowa using either `cargo run` or the compiled binary directly. Here are some example commands:

### Create a new account

```
cargo run -- create-account
```

This will generate a new keypair and save it to `~/.config/sowa/keypair.json`.

### Check account balance

```
cargo run -- balance <PUBKEY>
```

Replace `<PUBKEY>` with the address you want to check.

### Send SOL

```
cargo run -- send <TO_PUBKEY> <AMOUNT>
```

This sends the specified amount of SOL to the given address.

### View transaction history

```
cargo run -- history
```

This command displays the transaction history for the current account.

### Configure RPC URL

View current RPC URL:
```
cargo run -- config
```

Update RPC URL:
```
cargo run -- config <NEW_RPC_URL>
```

## Configuration

Sowa stores its configuration in `~/.config/sowa/config.json`. The default RPC URL is set to the Solana devnet. You can change this to mainnet-beta or testnet as needed.

## Security Notes

- The keypair is stored locally at `~/.config/sowa/keypair.json`. Keep this file safe and do not share it with anyone.
- Always double-check addresses when sending transactions to avoid losing funds.