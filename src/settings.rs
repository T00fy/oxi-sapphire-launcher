use anyhow::{anyhow, Error};
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
