use anyhow::{Result, anyhow};

use crate::commands;
use crate::dispatch::types::CoreCommand;

pub(in crate::dispatch) fn matches(command: &CoreCommand) -> bool {
    matches!(
        command,
        CoreCommand::AddApp { .. }
            | CoreCommand::RemoveApp { .. }
            | CoreCommand::ListApp { .. }
            | CoreCommand::UpdateApp { .. }
    )
}

pub(in crate::dispatch) async fn dispatch(command: CoreCommand) -> Result<()> {
    match command {
        #[cfg(feature = "app")]
        CoreCommand::AddApp {
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
            commands::core::add::app::add(
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
        #[cfg(not(feature = "app"))]
        CoreCommand::AddApp { .. } => Err(crate::error::core_not_in_build("app")),
        #[cfg(feature = "app")]
        CoreCommand::RemoveApp {
            packages,
            compat_runtime,
        } => commands::core::remove::app::remove(packages, compat_runtime).await,
        #[cfg(not(feature = "app"))]
        CoreCommand::RemoveApp { .. } => Err(crate::error::core_not_in_build("app")),
        #[cfg(feature = "app")]
        CoreCommand::ListApp { compat_runtime } => {
            commands::core::list::app::list(compat_runtime).await
        }
        #[cfg(not(feature = "app"))]
        CoreCommand::ListApp { .. } => Err(crate::error::core_not_in_build("app")),
        #[cfg(feature = "app")]
        CoreCommand::UpdateApp {
            packages,
            install,
            compat_runtime,
        } => commands::core::update::app::update(packages, install, compat_runtime).await,
        #[cfg(not(feature = "app"))]
        CoreCommand::UpdateApp { .. } => Err(crate::error::core_not_in_build("app")),
        _ => Err(anyhow!(
            "package dispatcher routed a command to the wrong core owner (expected app)"
        )),
    }
}
