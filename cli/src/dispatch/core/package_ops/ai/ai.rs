use anyhow::{Result, anyhow};

use crate::commands;
use crate::dispatch::types::CoreCommand;

pub(in crate::dispatch) fn matches(command: &CoreCommand) -> bool {
    matches!(
        command,
        CoreCommand::AddAi { .. }
            | CoreCommand::RemoveAi { .. }
            | CoreCommand::ListAi { .. }
            | CoreCommand::UpdateAi { .. }
    )
}

pub(in crate::dispatch) async fn dispatch(command: CoreCommand) -> Result<()> {
    match command {
        #[cfg(feature = "ai")]
        CoreCommand::AddAi {
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
            commands::core::add::ai::add(
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
        #[cfg(not(feature = "ai"))]
        CoreCommand::AddAi { .. } => Err(crate::error::core_not_in_build("ai")),
        #[cfg(feature = "ai")]
        CoreCommand::RemoveAi {
            packages,
            compat_runtime,
        } => commands::core::remove::ai::remove(packages, compat_runtime).await,
        #[cfg(not(feature = "ai"))]
        CoreCommand::RemoveAi { .. } => Err(crate::error::core_not_in_build("ai")),
        #[cfg(feature = "ai")]
        CoreCommand::ListAi { compat_runtime } => {
            commands::core::list::ai::list(compat_runtime).await
        }
        #[cfg(not(feature = "ai"))]
        CoreCommand::ListAi { .. } => Err(crate::error::core_not_in_build("ai")),
        #[cfg(feature = "ai")]
        CoreCommand::UpdateAi {
            packages,
            install,
            compat_runtime,
        } => commands::core::update::ai::update(packages, install, compat_runtime).await,
        #[cfg(not(feature = "ai"))]
        CoreCommand::UpdateAi { .. } => Err(crate::error::core_not_in_build("ai")),
        _ => Err(anyhow!(
            "package dispatcher routed a command to the wrong core owner (expected ai)"
        )),
    }
}
