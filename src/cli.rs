use clap::{Parser, Subcommand};
use crate::settings::{LoginSettings, RegisterSettings};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}


#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Login operation
    Login(LoginSettings),
    /// Registration operation
    Register(RegisterSettings),
}