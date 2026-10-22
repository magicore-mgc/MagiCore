// Bare remove adapter — normalize the detected core and remove flags.
// Adapter remove bare — chuẩn hóa core đã nhận diện và các cờ remove.
use crate::commands::definitions::commands::bare::BareCommands;
use crate::dispatch::remove_mapping::{RemoveRequest, RemoveTarget, to_core_command};
use crate::dispatch::types::CoreCommand;

use super::error::BareRemoveError;
use crate::dispatch::bare::ecosystem::require_ecosystem;
use crate::dispatch::bare::error::unknown_core;

pub(in crate::dispatch) fn map(
    command: BareCommands,
    ecosystem: Option<&str>,
) -> anyhow::Result<CoreCommand> {
    match command {
        BareCommands::Remove {
            packages,
            no_install,
            compat_runtime,
        } => {
            let core = require_ecosystem("remove", ecosystem)?;
            let target = RemoveTarget::from_core_name(core).ok_or_else(|| unknown_core(core))?;
            Ok(to_core_command(
                target,
                RemoveRequest {
                    packages,
                    install_after_remove: !no_install,
                    compat_runtime,
                },
            ))
        }
        _ => Err(anyhow::Error::new(BareRemoveError::UnexpectedCommand)),
    }
}

#[cfg(test)]
#[path = "test/remove.rs"]
mod tests;
