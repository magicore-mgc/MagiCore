use anyhow::{Result, anyhow};

use crate::commands;
use crate::dispatch::types::CoreCommand;

pub(in crate::dispatch) fn matches(command: &CoreCommand) -> bool {
    matches!(
        command,
        CoreCommand::AddIot { .. }
            | CoreCommand::RemoveIot { .. }
            | CoreCommand::ListIot { .. }
            | CoreCommand::UpdateIot { .. }
    )
}

pub(in crate::dispatch) async fn dispatch(command: CoreCommand) -> Result<()> {
    match command {
        #[cfg(feature = "iot")]
        CoreCommand::AddIot {
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
            commands::core::add::iot::add(
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
        #[cfg(not(feature = "iot"))]
        CoreCommand::AddIot { .. } => Err(crate::error::core_not_in_build("iot")),
        #[cfg(feature = "iot")]
        CoreCommand::RemoveIot {
            packages,
            compat_runtime,
        } => commands::core::remove::iot::remove(packages, compat_runtime).await,
        #[cfg(not(feature = "iot"))]
        CoreCommand::RemoveIot { .. } => Err(crate::error::core_not_in_build("iot")),
        #[cfg(feature = "iot")]
        CoreCommand::ListIot { compat_runtime } => {
            commands::core::list::iot::list(compat_runtime).await
        }
        #[cfg(not(feature = "iot"))]
        CoreCommand::ListIot { .. } => Err(crate::error::core_not_in_build("iot")),
        #[cfg(feature = "iot")]
        CoreCommand::UpdateIot {
            packages,
            install,
            compat_runtime,
        } => commands::core::update::iot::update(packages, install, compat_runtime).await,
        #[cfg(not(feature = "iot"))]
        CoreCommand::UpdateIot { .. } => Err(crate::error::core_not_in_build("iot")),
        _ => Err(anyhow!(
            "package dispatcher routed a command to the wrong core owner (expected iot)"
        )),
    }
}
