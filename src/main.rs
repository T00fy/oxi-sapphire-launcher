use std::path::PathBuf;
use std::process;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use log::debug;
use physis::repository::Repository;
use rpassword::read_password;

use encryptor::encrypt_game_arg;

use crate::cli::{Cli, Commands};
use crate::client::{LoginAuthResponse, LoginServerResponse};
use crate::launcher::Launcher;
#[cfg(target_os = "linux")]
use crate::lutris_launcher::LutrisLauncher;
use crate::settings::{CoreSettings, ensure_core_settings_exists, load_core_settings};
#[cfg(target_os = "windows")]
use crate::windows_launcher::WindowsLauncher;

mod cli;
mod settings;
mod client;
mod encryptor;
mod launcher;
#[cfg(target_os = "linux")]
mod lutris_launcher;
#[cfg(target_os = "windows")]
mod windows_launcher;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    ensure_core_settings_exists()?;
    let core_settings = load_core_settings()?;
    let cli = Cli::parse();

    match &cli.command {
        Commands::Login(login_settings) => {
            let password = get_password(&login_settings.password)?;
            let login_response = client::send_login_request(&core_settings, login_settings, password.as_str()).await?;
            handle_game_launch(&core_settings, login_response, login_settings.exit_on_auth).await?;
        }
        Commands::Register(register_settings) => {
            let password = get_password(&register_settings.password)?;
            let register_response = client::send_register_request(&core_settings, register_settings, password.as_str()).await?;
            println!("Account created successfully.");
            if !register_settings.no_login_on_register {
                println!("Logging in with the newly created account...");
                handle_game_launch(&core_settings, register_response, register_settings.exit_on_auth).await?;
            }
        },
    }
    Ok(())
}

/// Handles game launching logic after a successful login.
async fn handle_game_launch(core_settings: &CoreSettings, login_response: LoginServerResponse, exit_on_auth: bool) -> Result<()> {
    let login_auth = LoginAuthResponse {
        sid: login_response.s_id,
        lobby_host: login_response.lobby_host,
        frontier_host: login_response.frontier_host,
        ..LoginAuthResponse::default()
    };
    let game_args = get_game_args(&login_auth, core_settings)
        .map_err(|e| anyhow!("Failed to get game args: {}", e))?;

    if exit_on_auth {
        println!("{}", game_args);
        process::exit(0);
    }

    #[cfg(target_os = "linux")]
        let launcher = LutrisLauncher;

    #[cfg(target_os = "windows")]
        let launcher = WindowsLauncher;
    launcher.launch_game(&game_args, &core_settings.game_dir)?;
    println!("Game launch initiated. Please check the game window to ensure it started successfully.");
    println!("Exiting Oxi Launcher.");

    Ok(())
}


/// Prompts for password if not provided.
fn get_password(password: &Option<String>) -> Result<String> {
    match password {
        Some(p) => Ok(p.clone()),
        None => {
            println!("Enter your password:");
            let entered_password = read_password().context("Failed to read password")?;
            if entered_password.is_empty() {
                Err(anyhow!("Password cannot be empty"))
            } else {
                Ok(entered_password)
            }
        }
    }
}

fn get_game_args(auth: &LoginAuthResponse, core_settings: &CoreSettings) -> Result<String> {
    //from root, BaseRepository == ./game/sqpack/ffxiv
    //from root, expansions == ./game/sqpack/ex1 etc
    let mut path = PathBuf::from(&core_settings.game_dir);
    path.push("game");
    path.push("sqpack");
    path.push("ffxiv");
    let base_repository = Repository::from_existing(path.to_str().unwrap()).unwrap();
    let game_args = vec![
        ("DEV.DataPathType".to_string(), "1".to_string()),
        ("DEV.UseSqPack".to_string(), "1".to_string()),
        ("DEV.MaxEntitledExpansionID".to_string(), auth.max_expansion.to_string()),
        ("DEV.TestSID".to_string(), auth.sid.clone()),
        ("SYS.Region".to_string(), auth.region.to_string()),
        ("language".to_string(), auth.language.to_string()),
        ("ver".to_string(), base_repository.version.unwrap()),
        ("DEV.GMServerHost".to_string(), auth.frontier_host.clone()),
    ];
    let mut game_args_with_lobby = game_args;
    for i in 1..=8 {
        game_args_with_lobby.push((format!("DEV.LobbyHost0{}", i), auth.lobby_host.clone()));
        game_args_with_lobby.push((format!("DEV.LobbyPort0{}", i), "54994".to_string())); //TODO: sapphire does not provide a port, unfortunately. Change this when Sapphire has been fixed upstream.
    }

    // Join the arguments into a string
    let arg_joined = game_args_with_lobby
        .into_iter()
        .map(|(key, value)| format!(" /{} ={}", key, value))
        .collect::<Vec<String>>()
        .join("");
    debug!("Game_args: {}", arg_joined);

    // Encrypt the arguments
    encrypt_game_arg(&arg_joined)
}