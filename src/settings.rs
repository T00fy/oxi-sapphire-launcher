use std::{env, fs, process};
use std::fs::File;
use std::io::Read;
use anyhow::{anyhow, Context, Error};
use clap::Parser;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(crate) struct CoreSettings {
    pub(crate) game_dir: String,
    pub(crate) frontier_ip: String,
    pub(crate) frontier_port: u16,
    pub(crate) frontier_scheme: String,
}

impl CoreSettings {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.frontier_ip.is_empty() {
            return Err(anyhow!("frontier_ip is missing in core_settings.json"));
        }
        if self.frontier_port == 0 {
            return Err(anyhow!("frontier_port is missing in core_settings.json"));
        }
        if self.frontier_scheme.is_empty() {
            return Err(anyhow!("frontier_scheme is missing in core_settings.json"));
        }
        if self.game_dir.is_empty() {
            return Err(anyhow!("game_dir is missing in core_settings.json"));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Parser)]
pub(crate) struct LoginSettings {
    #[clap(long)]
    pub(crate) username: String,
    #[clap(long)]
    pub(crate) password: Option<String>,
    #[clap(long, default_value = "/sapphire-api/lobby/login")]
    pub(crate) endpoint: String,
    #[clap(long)]
    pub(crate) exit_on_auth: bool,
}

#[derive(Debug, Deserialize, Parser)]
pub(crate) struct RegisterSettings {
    #[clap(long)]
    pub(crate) username: String,
    #[clap(long)]
    pub(crate) password: String,
    #[clap(long, default_value = "/sapphire-api/lobby/createAccount")]
    pub(crate) endpoint: String,
}


pub(crate) fn load_core_settings() -> anyhow::Result<CoreSettings, Error> {
    let mut file = File::open("core_settings.json")
        .context("Failed to open core_settings.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Failed to read core_settings.json")?;
    let core_settings: CoreSettings = serde_json::from_str(&contents)
        .context("Failed to parse core_settings.json")?;

    // Validate that all fields are populated
    core_settings.validate()?;

    Ok(core_settings)
}

pub(crate) fn ensure_core_settings_exists() -> anyhow::Result<(), std::io::Error> {
    let mut exe_path = env::current_exe()?;
    exe_path.pop(); // remove the executable name, leaving just the path
    let config_path = exe_path.join("core_settings.json");

    if !config_path.exists() {
        println!("core_settings does not exist. Creating default core_settings.json");
        let default_config = include_str!("../resources/core_settings.json");
        fs::write(&config_path, default_config)?;
        println!("Modify core_settings.json as needed and rerun oxi launcher");
        process::exit(2);
    }

    Ok(())
}