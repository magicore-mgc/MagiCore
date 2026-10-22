// Bare update adapter — normalize the detected core and update fields.
// Adapter update bare — chuẩn hóa core đã nhận diện và field update.
use crate::commands::definitions::commands::bare::BareCommands;
use crate::dispatch::types::CoreCommand;
use crate::dispatch::update_mapping::{UpdateRequest, UpdateTarget, to_core_command};

use super::error::BareUpdateError;
use crate::dispatch::bare::ecosystem::require_ecosystem;
use crate::dispatch::bare::error::unknown_core;

pub(in crate::dispatch) fn map(
    command: BareCommands,
    ecosystem: Option<&str>,
) -> anyhow::Result<CoreCommand> {
    match command {
        BareCommands::Update {
            packages,
            install,
            compat_runtime,
        } => {
            let core = require_ecosystem("update", ecosystem)?;
            let target = UpdateTarget::from_core_name(core).ok_or_else(|| unknown_core(core))?;
            Ok(to_core_command(
                target,
                UpdateRequest {
                    packages,
                    install,
                    compat_runtime,
                },
            ))
        }
        _ => Err(anyhow::Error::new(BareUpdateError::UnexpectedCommand)),
    }
}

#[cfg(test)]
#[path = "test/update.rs"]
mod tests;
