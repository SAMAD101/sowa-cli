use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use dirs::home_dir;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub rpc_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            rpc_url: "https://api.devnet.solana.com".to_string(),
        }
    }
}

fn get_config_path() -> Result<std::path::PathBuf> {
    let mut path = home_dir().ok_or_else(|| anyhow!("Unable to get home directory"))?;
    path.extend(&[".config", "sowa", "config.json"]);
    Ok(path)
}

pub fn get_config() -> Result<Config> {
    let path = get_config_path()?;
    if path.exists() {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_json::from_str(&contents)?)
    } else {
        let config = Config::default();
        save_config(&config)?;
        Ok(config)
    }
}

pub fn save_config(config: &Config) -> Result<()> {
    let path = get_config_path()?;
    create_dir_all(path.parent().unwrap())?;
    let mut file = File::create(path)?;
    let contents = serde_json::to_string_pretty(config)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}