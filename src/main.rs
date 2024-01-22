mod cli;
mod settings;
mod oxi_file_config;

use clap::Parser;
use crate::cli::{Cli, Commands};
use config::{Config, File, FileFormat};
use log::debug;
use crate::oxi_file_config::OxiFileConfig;
use crate::settings::*;

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    // Step 1: Create and merge settings
    let file_lookup = Config::builder()
        .add_source(File::with_name("settings").format(FileFormat::Ini)).build();

    // Step 2: Convert settings to OxiFileConfig after merge
    let file_config: Option<OxiFileConfig> = match file_lookup {
        Ok(config) => {
            config.try_deserialize().map_err(|e| {
                debug!("Failed to serialize file: {}", e);
                e
            }).ok()
        },
        Err(e) => {
            debug!("Failed to load 'settings' file: {}", e); //TODO why isn't this showing anything in terminal
            None
        }
    };

    match &cli.command {
        Commands::Login(login_settings) => {
            // Merge login settings from the file and CLI
            let file_login_settings = file_config.as_ref().and_then(|config| config.login.as_ref());
            let merged_login_settings = merge_login_settings(
                file_login_settings,
                login_settings,
            );
            if let Err(error) = validate_login_settings(&cli.core, &merged_login_settings) {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
            debug!("Login command received. Settings {:#?}", &merged_login_settings);
            // Implement the login logic here
            // You have merged_login_settings and cli.core to work with
        }

        Commands::Register(_register_settings) => {
            println!("Register command received");
            // Implement the registration logic here
            // Directly use register_settings and cli.core, as there are no settings from the file for register
        }
    }
}

///
fn merge_login_settings(file_login: Option<&LoginSettings>, cli_login: &LoginSettings) -> LoginSettings {
    LoginSettings {
        username: if !cli_login.username.is_empty() {
            Some(cli_login.username.clone())
        } else {
            file_login.and_then(|l| Some(l.username.clone()))
        }.unwrap_or_default(),

        password: if !cli_login.password.is_empty() {
            Some(cli_login.password.clone())
        } else {
            file_login.and_then(|l| Some(l.password.clone()))
        }.unwrap_or_default(),

        endpoint: if !cli_login.endpoint.is_empty() {
            Some(cli_login.endpoint.clone())
        } else {
            file_login.and_then(|l| Some(l.endpoint.clone()))
        }.unwrap_or_default(),

        exit_on_auth: cli_login.exit_on_auth,
    }
}


fn validate_login_settings(core: &CoreSettings, login: &LoginSettings) -> Result<(), String> {
    // Validate CoreSettings
    if core.game_dir.is_empty() {
        return Err("Game directory is required.".to_string());
    }

    if core.lobby_ip.is_empty() {
        return Err("Lobby IP is required.".to_string());
    }

    if core.lobby_port == 0 {
        return Err("Lobby port is required and must be non-zero.".to_string());
    }

    if core.lobby_scheme.is_empty() {
        return Err("Lobby scheme is required.".to_string());
    }

    // Validate LoginSettings
    if login.username.is_empty() {
        return Err("Username for login is required.".to_string());
    }

    if login.password.is_empty() {
        return Err("Password for login is required.".to_string());
    }

    if login.endpoint.is_empty() {
        return Err("Login endpoint is required.".to_string());
    }

    Ok(())
}



