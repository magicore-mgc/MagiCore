use anyhow::{Result, anyhow};

use crate::commands;
use crate::dispatch::types::CoreCommand;

pub(in crate::dispatch) fn matches(command: &CoreCommand) -> bool {
    matches!(
        command,
        CoreCommand::AddClo { .. }
            | CoreCommand::RemoveClo { .. }
            | CoreCommand::ListClo { .. }
            | CoreCommand::UpdateClo { .. }
    )
}

pub(in crate::dispatch) async fn dispatch(command: CoreCommand) -> Result<()> {
    match command {
        #[cfg(feature = "clo")]
        CoreCommand::AddClo {
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
            commands::core::add::clo::add(
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
        #[cfg(not(feature = "clo"))]
        CoreCommand::AddClo { .. } => Err(crate::error::core_not_in_build("clo")),
        #[cfg(feature = "clo")]
        CoreCommand::RemoveClo {
            packages,
            compat_runtime,
        } => commands::core::remove::clo::remove(packages, compat_runtime).await,
        #[cfg(not(feature = "clo"))]
        CoreCommand::RemoveClo { .. } => Err(crate::error::core_not_in_build("clo")),
        #[cfg(feature = "clo")]
        CoreCommand::ListClo { compat_runtime } => {
            commands::core::list::clo::list(compat_runtime).await
        }
        #[cfg(not(feature = "clo"))]
        CoreCommand::ListClo { .. } => Err(crate::error::core_not_in_build("clo")),
        #[cfg(feature = "clo")]
        CoreCommand::UpdateClo {
            packages,
            install,
            compat_runtime,
        } => commands::core::update::clo::update(packages, install, compat_runtime).await,
        #[cfg(not(feature = "clo"))]
        CoreCommand::UpdateClo { .. } => Err(crate::error::core_not_in_build("clo")),
        _ => Err(anyhow!(
            "package dispatcher routed a command to the wrong core owner (expected clo)"
        )),
    }
}
