use anyhow::{Result, anyhow};
use solana_sdk::signature::Keypair;
use std::fs::{File, create_dir_all};
use std::io::{Write, Read};
use dirs::home_dir;

fn get_keypair_path() -> Result<std::path::PathBuf> {
    let mut path = home_dir().ok_or_else(|| anyhow!("Unable to get home directory"))?;
    path.push(".config");
    path.push("sowa");
    path.push("keypair.json");
    Ok(path)
}

pub fn save_keypair(keypair: &Keypair) -> Result<()> {
    let path = get_keypair_path()?;
    create_dir_all(path.parent().unwrap())?;
    let mut file = File::create(path)?;
    file.write_all(&keypair.to_bytes())?;
    Ok(())
}

pub fn load_keypair() -> Result<Keypair> {
    let path = get_keypair_path()?;
    let mut file = File::open(path)?;
    let mut bytes = vec![0u8; 64];
    file.read_exact(&mut bytes)?;
    Ok(Keypair::from_bytes(&bytes)?)
}