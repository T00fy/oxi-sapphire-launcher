use std::path::Path;
use std::process::{Command};
use anyhow::{anyhow, Error};
use crate::launcher::Launcher;

pub(crate) struct WindowsLauncher;

impl Launcher for WindowsLauncher {
    fn launch_game(&self, game_args: &str, game_dir: &str) -> Result<(), Error> {
        // Construct the path to the game executable
        let game_exe_path = Path::new(game_dir).join("game").join("ffxiv_dx11.exe");

        // Verify that the executable exists
        if !game_exe_path.exists() {
            return Err(anyhow!("Game executable not found at expected location: {:?}", game_exe_path));
        }

        // Launch the game executable with game_args
        let status = Command::new(game_exe_path)
            .arg(game_args)
            .spawn() // Spawn the process without waiting for it to finish
            .and_then(|mut child| child.wait()) // Wait for the game to launch successfully
            .map_err(|e| anyhow!("Failed to start game: {}", e))?;

        if !status.success() {
            return Err(anyhow!("Game exited with non-zero status: {:?}", status.code()));
        }

        Ok(())
    }
}