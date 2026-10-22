use anyhow::{Result, anyhow};

use crate::commands;
use crate::dispatch::types::CoreCommand;

pub(in crate::dispatch) fn matches(command: &CoreCommand) -> bool {
    matches!(
        command,
        CoreCommand::AddLib { .. }
            | CoreCommand::RemoveLib { .. }
            | CoreCommand::ListLib { .. }
            | CoreCommand::UpdateLib { .. }
    )
}

pub(in crate::dispatch) async fn dispatch(command: CoreCommand) -> Result<()> {
    match command {
        #[cfg(feature = "lib")]
        CoreCommand::AddLib {
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
            commands::core::add::library::add(
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
        #[cfg(not(feature = "lib"))]
        CoreCommand::AddLib { .. } => Err(crate::error::core_not_in_build("lib")),
        #[cfg(feature = "lib")]
        CoreCommand::RemoveLib {
            packages,
            compat_runtime,
        } => commands::core::remove::library::remove(packages, compat_runtime).await,
        #[cfg(not(feature = "lib"))]
        CoreCommand::RemoveLib { .. } => Err(crate::error::core_not_in_build("lib")),
        #[cfg(feature = "lib")]
        CoreCommand::ListLib { compat_runtime } => {
            commands::core::list::library::list(compat_runtime).await
        }
        #[cfg(not(feature = "lib"))]
        CoreCommand::ListLib { .. } => Err(crate::error::core_not_in_build("lib")),
        #[cfg(feature = "lib")]
        CoreCommand::UpdateLib {
            packages,
            install,
            compat_runtime,
        } => commands::core::update::library::update(packages, install, compat_runtime).await,
        #[cfg(not(feature = "lib"))]
        CoreCommand::UpdateLib { .. } => Err(crate::error::core_not_in_build("lib")),
        _ => Err(anyhow!(
            "package dispatcher routed a command to the wrong core owner (expected lib)"
        )),
    }
}
