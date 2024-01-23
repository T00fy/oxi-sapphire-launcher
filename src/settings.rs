use clap::Parser;
use serde::Deserialize;
#[derive(Debug, Deserialize, Parser)]
pub(crate) struct CoreSettings {
    #[clap(long)]
    pub(crate) game_dir: String,
    #[clap(long)] //used for both login and register
    pub(crate) frontier_ip: String,
    #[clap(long)] //used for both login and register
    pub(crate) frontier_port: u16,
    #[clap(long, default_value = "http")]
    pub(crate) frontier_scheme: String,
}

#[derive(Debug, Deserialize, Parser)]
pub(crate) struct LoginSettings {
    #[clap(long)]
    pub(crate) username: String,
    #[clap(long)]
    pub(crate) password: String,
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
