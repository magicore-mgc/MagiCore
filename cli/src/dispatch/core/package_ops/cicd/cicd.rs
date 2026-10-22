use anyhow::{Result, anyhow};

use crate::commands;
use crate::dispatch::types::CoreCommand;

pub(in crate::dispatch) fn matches(command: &CoreCommand) -> bool {
    matches!(
        command,
        CoreCommand::AddCicd { .. }
            | CoreCommand::RemoveCicd { .. }
            | CoreCommand::ListCicd { .. }
            | CoreCommand::UpdateCicd { .. }
    )
}

pub(in crate::dispatch) async fn dispatch(command: CoreCommand) -> Result<()> {
    match command {
        #[cfg(feature = "cicd")]
        CoreCommand::AddCicd {
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
            commands::core::add::cicd::add(
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
        #[cfg(not(feature = "cicd"))]
        CoreCommand::AddCicd { .. } => Err(crate::error::core_not_in_build("cicd")),
        #[cfg(feature = "cicd")]
        CoreCommand::RemoveCicd {
            packages,
            compat_runtime,
        } => commands::core::remove::cicd::remove(packages, compat_runtime).await,
        #[cfg(not(feature = "cicd"))]
        CoreCommand::RemoveCicd { .. } => Err(crate::error::core_not_in_build("cicd")),
        #[cfg(feature = "cicd")]
        CoreCommand::ListCicd { compat_runtime } => {
            commands::core::list::cicd::list(compat_runtime).await
        }
        #[cfg(not(feature = "cicd"))]
        CoreCommand::ListCicd { .. } => Err(crate::error::core_not_in_build("cicd")),
        #[cfg(feature = "cicd")]
        CoreCommand::UpdateCicd {
            packages,
            install,
            compat_runtime,
        } => commands::core::update::cicd::update(packages, install, compat_runtime).await,
        #[cfg(not(feature = "cicd"))]
        CoreCommand::UpdateCicd { .. } => Err(crate::error::core_not_in_build("cicd")),
        _ => Err(anyhow!(
            "package dispatcher routed a command to the wrong core owner (expected cicd)"
        )),
    }
}
