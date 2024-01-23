use std::fs;
use std::process::{Command, Stdio};

use anyhow::{anyhow, Error};
use serde_json::from_str;
use serde_yaml::Value;

use crate::launcher::Launcher;

pub(crate) struct LinuxLauncher;


impl Launcher for LinuxLauncher {
    fn launch_game(&self, game_args: &str) -> Result<(), Error> {
        if !LinuxLauncher::is_lutris_installed()? {
            return Err(anyhow!("Lutris must be installed on the system."));
        }

        let game_id = LinuxLauncher::get_ffxivsapphire_game_id()?;
        LinuxLauncher::update_game_args_in_yaml(game_args)?;
        LinuxLauncher::launch_game_with_lutris(&game_id)?;

        Ok(())
    }
}

impl LinuxLauncher {
    fn is_lutris_installed() -> Result<bool, Error> {
        let output = Command::new("lutris")
            .arg("--version")
            .output()?;

        Ok(output.status.success())
    }

    fn get_ffxivsapphire_game_id() -> Result<i32, Error> {
        let output = Command::new("sh")
            .arg("-c")
            .arg("lutris -l -j 2>/dev/null")
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.trim().is_empty() {
            return Err(anyhow!("The command 'lutris -l -j' did not produce any stdout output. Is Lutris installed and configured correctly?"));
        }

        // Parse the JSON string
        let games: Vec<Value> = from_str(&output_str)
            .map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;

        // Look for the game with the slug "ffxivsapphire" and extract its ID
        let game_id = games.iter()
            .find(|game| game["slug"] == "ffxivsapphire")
            .and_then(|game| game["id"].as_i64())
            .ok_or_else(|| anyhow!("(FFXIV Heavensward 3.3) with the name 'FFXIVSAPPHIRE' was not configured in Lutris"))?;

        Ok(game_id as i32)
    }

    fn update_game_args_in_yaml(game_args: &str) -> Result<(), Error> {
        let home_dir = std::env::var("HOME")?;
        let game_config_pattern = format!("{}/.config/lutris/games/ffxivsapphire*.yml", home_dir);
        let game_config_paths = glob::glob(&game_config_pattern)?; // glob library can be used to find files matching a pattern

        for game_config_path in game_config_paths {
            match game_config_path {
                Ok(path) => {
                    // Read the YAML file
                    let contents = fs::read_to_string(&path)?;
                    let mut yaml: Value = serde_yaml::from_str(&contents)?;

                    // Navigate and update 'args' field under 'game'
                    if let Some(game_section) = yaml.get_mut("game") {
                        game_section["args"] = Value::from(game_args);
                    }

                    // Write the updated YAML back to the file
                    let new_contents = serde_yaml::to_string(&yaml)?;
                    fs::write(&path, new_contents)?;
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        Ok(())
    }

    fn launch_game_with_lutris(game_id: &i32) -> Result<(), Error> {
        Command::new("env")
            .arg("LUTRIS_SKIP_INIT=1")
            .arg("lutris")
            .arg(format!("lutris:rungameid/{}", game_id))
            .stdout(Stdio::null())  // Redirect stdout to /dev/null
            .stderr(Stdio::null())  // Redirect stderr to /dev/null
            .spawn()?; // Spawn the process without waiting for it to finish

        Ok(())
    }
}



