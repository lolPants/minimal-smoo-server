use std::sync::Arc;

use color_eyre::eyre::bail;
use color_eyre::Result;
use tracing::info;

use super::commands::{Command, ConfigCommand, MoonCommand};
use crate::config::SharedConfig;
use crate::server::Server;

pub(super) async fn handle_command(
    command: Command,
    server: Arc<Server>,
    config: SharedConfig,
) -> Result<HandleResult> {
    match command {
        Command::Exit => Ok(HandleResult::Exit),

        Command::Config(ConfigCommand::Reload) => {
            let mut config = config.write().await;

            config.reload().await?;
            info!("Loaded config from file");

            Ok(HandleResult::Ok)
        }

        Command::Config(ConfigCommand::Save) => {
            let config = config.read().await;

            config.save().await?;
            info!("Force saved config to file");

            Ok(HandleResult::Ok)
        }

        Command::List => {
            // TODO
            bail!("not yet implemented")
        }

        Command::Send {
            stage,
            scenario,
            warp_id,
            players,
        } => {
            // TODO
            bail!("not yet implemented")
        }

        Command::SendAll {
            stage,
            scenario,
            warp_id,
        } => {
            // TODO
            bail!("not yet implemented")
        }

        Command::Moon(MoonCommand::List) => {
            // TODO
            bail!("not yet implemented")
        }

        Command::Moon(MoonCommand::Sync) => {
            server.sync_moons().await?;
            info!("Synced moons to all players");

            Ok(HandleResult::Ok)
        }

        Command::Moon(MoonCommand::Give { id, players }) => {
            // TODO
            bail!("not yet implemented")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum HandleResult {
    Ok,
    Exit,
}
