use anyhow::{Result, anyhow};

use crate::commands;
use crate::dispatch::types::CoreCommand;

pub(in crate::dispatch) fn matches(command: &CoreCommand) -> bool {
    matches!(
        command,
        CoreCommand::AddGame { .. }
            | CoreCommand::RemoveGame { .. }
            | CoreCommand::ListGame { .. }
            | CoreCommand::UpdateGame { .. }
    )
}

pub(in crate::dispatch) async fn dispatch(command: CoreCommand) -> Result<()> {
    match command {
        #[cfg(feature = "game")]
        CoreCommand::AddGame {
            packages,
            dev,
            exact,
            optional,
            peer,
            no_save,
            global,
            compat_runtime,
            version,
        } => {
            commands::core::add::game::add(
                packages,
                version,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
            )
            .await
        }
        #[cfg(not(feature = "game"))]
        CoreCommand::AddGame { .. } => Err(crate::error::core_not_in_build("game")),
        #[cfg(feature = "game")]
        CoreCommand::RemoveGame {
            packages,
            compat_runtime,
        } => commands::core::remove::game::remove(packages, compat_runtime).await,
        #[cfg(not(feature = "game"))]
        CoreCommand::RemoveGame { .. } => Err(crate::error::core_not_in_build("game")),
        #[cfg(feature = "game")]
        CoreCommand::ListGame { compat_runtime } => {
            commands::core::list::game::list(compat_runtime).await
        }
        #[cfg(not(feature = "game"))]
        CoreCommand::ListGame { .. } => Err(crate::error::core_not_in_build("game")),
        #[cfg(feature = "game")]
        CoreCommand::UpdateGame {
            packages,
            install,
            compat_runtime,
        } => commands::core::update::game::update(packages, install, compat_runtime).await,
        #[cfg(not(feature = "game"))]
        CoreCommand::UpdateGame { .. } => Err(crate::error::core_not_in_build("game")),
        _ => Err(anyhow!(
            "package dispatcher routed a command to the wrong core owner (expected game)"
        )),
    }
}
