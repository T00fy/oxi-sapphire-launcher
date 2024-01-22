use clap::Parser;
use serde::Deserialize;
use serde_with::serde_as;
#[serde_as]
#[derive(Debug, Deserialize, Parser)]  // Added Parser for clap support
pub(crate) struct CoreSettings {
    #[clap(long)]
    pub(crate) game_dir: String,
    #[clap(long)]
    pub(crate) lobby_ip: String,
    #[clap(long)]
    pub(crate) lobby_port: u16,
    #[serde(default = "default_scheme")]
    #[clap(long, default_value_t = String::from("http"))]
    pub(crate) lobby_scheme: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Parser)] // Added Parser for clap support
pub(crate) struct LoginSettings {
    #[clap(long)]
    pub(crate) username: String,
    #[clap(long)]
    pub(crate) password: String,
    #[serde(default = "default_login_endpoint")]
    #[clap(long, default_value = "/sapphire-api/lobby/login")]
    pub(crate) endpoint: String,
    #[clap(long)]
    pub(crate) exit_on_auth: bool,
}

#[serde_as]
#[derive(Debug, Deserialize, Parser)] // Added Parser for clap support
pub(crate) struct RegisterSettings {
    #[clap(long)]
    pub(crate) username: String,
    #[clap(long)]
    pub(crate) password: String,
    #[serde(default = "default_register_endpoint")]
    #[clap(long, default_value = "/sapphire-api/lobby/createAccount")]
    pub(crate) endpoint: String,
}

fn default_login_endpoint() -> String {
    "/sapphire-api/lobby/login".to_string()
}

fn default_register_endpoint() -> String {
    "/sapphire-api/lobby/createAccount".to_string()
}

fn default_scheme() -> String {
    "http".to_string()
}
