use anyhow::Error;

pub(crate) trait Launcher {
    fn launch_game(&self, game_args: &str, game_dir: &str) -> Result<(), Error>;
}
