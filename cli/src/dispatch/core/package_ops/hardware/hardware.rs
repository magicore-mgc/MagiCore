use anyhow::{Result, anyhow};

use crate::commands;
use crate::dispatch::types::CoreCommand;

pub(in crate::dispatch) fn matches(command: &CoreCommand) -> bool {
    matches!(
        command,
        CoreCommand::AddHardware { .. }
            | CoreCommand::RemoveHardware { .. }
            | CoreCommand::ListHardware { .. }
            | CoreCommand::UpdateHardware { .. }
    )
}

pub(in crate::dispatch) async fn dispatch(command: CoreCommand) -> Result<()> {
    match command {
        #[cfg(feature = "hardware")]
        CoreCommand::AddHardware {
            packages,
            compat_runtime,
            version,
        } => commands::core::add::hardware::add(packages, compat_runtime, version).await,
        #[cfg(not(feature = "hardware"))]
        CoreCommand::AddHardware { .. } => Err(crate::error::core_not_in_build("hardware")),
        #[cfg(feature = "hardware")]
        CoreCommand::RemoveHardware {
            packages,
            compat_runtime,
        } => commands::core::remove::hardware::remove(packages, compat_runtime).await,
        #[cfg(not(feature = "hardware"))]
        CoreCommand::RemoveHardware { .. } => Err(crate::error::core_not_in_build("hardware")),
        #[cfg(feature = "hardware")]
        CoreCommand::ListHardware { compat_runtime } => {
            commands::core::list::hardware::list(compat_runtime).await
        }
        #[cfg(not(feature = "hardware"))]
        CoreCommand::ListHardware { .. } => Err(crate::error::core_not_in_build("hardware")),
        #[cfg(feature = "hardware")]
        CoreCommand::UpdateHardware {
            packages,
            install,
            compat_runtime,
        } => commands::core::update::hardware::update(packages, install, compat_runtime).await,
        #[cfg(not(feature = "hardware"))]
        CoreCommand::UpdateHardware { .. } => Err(crate::error::core_not_in_build("hardware")),
        _ => Err(anyhow!(
            "package dispatcher routed a command to the wrong core owner (expected hardware)"
        )),
    }
}
