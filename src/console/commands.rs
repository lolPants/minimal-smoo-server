use clap::Parser;

use super::Stage;

#[derive(Debug, Parser)]
#[clap(
    disable_help_flag = true,
    disable_version_flag = true,
    no_binary_name = true
)]
pub enum Command {
    #[clap(subcommand)]
    Config(ConfigCommand),

    /// List all currently connected players
    List,

    #[clap(subcommand)]
    Moon(MoonCommand),

    /// Send player(s) to a stage
    #[clap(allow_negative_numbers = true)]
    Send {
        stage: Stage,
        scenario: i8,
        warp_id: String,
        players: Vec<String>,
    },

    /// Send all players to a stage
    #[clap(alias = "sendall", allow_negative_numbers = true)]
    SendAll {
        stage: Stage,
        scenario: i8,

        #[clap(default_value = "")]
        warp_id: String,
    },

    /// Stop the server and exit
    #[clap(alias = "quit", alias = "stop", alias = "q")]
    Exit,
}

#[derive(Debug, Parser)]
pub enum ConfigCommand {
    /// Reload config from file
    Reload,

    /// Force save current config to disk
    Save,
}

#[derive(Debug, Parser)]
pub enum MoonCommand {
    /// List all currently collected moons
    List,

    /// Sync moons to all connected players
    Sync,

    /// Reload moons from moon file (if persistence is enabled)
    Reload,

    Clear,

    /// Manually add a specific moon to
    Add {
        id: i32,
    },
}
