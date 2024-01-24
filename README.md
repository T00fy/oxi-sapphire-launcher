# Oxi Sapphire Launcher

Oxi Sapphire Launcher is a cross-platform custom CLI launcher for Final Fantasy XIV, tailored for use with custom Sapphire servers.

It streamlines the process of setting up and launching the game with custom configurations.

## Features

- **Custom Game Configuration**: Configure your game settings to connect with custom Sapphire servers easily.
- **Secure Password Handling**: Password input is prompted to avoid saving sensitive information in shell history.
- **Cross-Platform Support**: Compatible with both Linux and Windows operating systems.
- **CLI-Based**: Simple command-line interface for straightforward setup and launching.

## Getting Started

Below are the instructions to get a copy of the project running on your machine for development and testing purposes.

### Prerequisites

- [Rust Programming Language](https://www.rust-lang.org/tools/install)
- Final Fantasy XIV 3.3 Installed

1. **Install FFXIV 2.0**: For linux, Use wine to run the game installer. You might be able to find this on archive.org. Otherwise on windows just install normally.
2. **Apply Patches**: You'll need to manually apply patches to upgrade to version 3.3 (Heavensward). Use the XIVLauncher.PatchInstaller.exe from FFXIVQuickLauncher to apply patches. For linux, this must also be run in wine. You will need to find the patch files yourself.

### Installation

1. Clone the repository:

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Find the executable in the `target/release` directory.

### Usage

1. On the first run, the launcher will create a `core_settings.json` file, printing out the path and then exit. You'll need to modify this file with your specific configurations.

2. Launch the game using:
    ```sh
    ./oxi-sapphire-launcher [COMMAND] [OPTIONS]
    ```

   For detailed command and options, use the `--help` flag:
    ```sh
    ./oxi-sapphire-launcher --help
    ```

## Configuration

Configure the launcher using the `core_settings.json` file. Below is the default configuration:

```json
{
  "game_dir": "C:\\Program Files (x86)\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn",
  "frontier_ip": "127.0.0.1",
  "frontier_port": 8080,
  "frontier_scheme": "http"
}
```

## Additional Steps for Linux Users

The Linux implementation relies on Lutris for managing game configurations and dependencies.
The reason Lutris was decided on is that it handles configuration for the base implementation of Wine easily. If you were just to use wine, the frame rate significantly drops for unknown reasons.

Steps:
1. **Install Lutris**: Lutris simplifies the process of managing graphical settings and dependencies.
2. **Configure Lutris**: Create a custom game in Lutris named 'FFXIVSAPPHIRE' that targets ffxiv_dx11.exe. Other settings can remain at their default values.

## Sapphire support 

Oxi Sapphire Launcher is designed for the master branch of the Sapphire Server, exclusively targeting FFXIV version 3.3 Heavensward.

## Contributions

Contributions are welcome. Feel free to fork the project, create your feature branch, commit your changes, and open a pull request.

## Acknowledgements

[Sapphire](https://github.com/SapphireServer/Sapphire)

[Astra](https://sr.ht/~redstrate/astra/)

[Physis](https://sr.ht/~redstrate/physis/)
