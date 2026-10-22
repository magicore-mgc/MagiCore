// Bare list adapter — convert the detected core and compatibility option to a request.
// Adapter list bare — đổi core phát hiện và cờ tương thích thành request.
use crate::commands::definitions::commands::bare::BareCommands;
use crate::dispatch::list_mapping::{ListRequest, ListTarget, to_core_command};
use crate::dispatch::types::CoreCommand;

use super::error::BareListError;
use crate::dispatch::bare::ecosystem::require_ecosystem;
use crate::dispatch::bare::error::unknown_core;

pub(in crate::dispatch) fn map(
    command: BareCommands,
    ecosystem: Option<&str>,
) -> anyhow::Result<CoreCommand> {
    match command {
        BareCommands::List { compat_runtime } => {
            let core = require_ecosystem("list", ecosystem)?;
            let target = ListTarget::from_core_name(core).ok_or_else(|| unknown_core(core))?;
            Ok(to_core_command(target, ListRequest { compat_runtime }))
        }
        _ => Err(anyhow::Error::new(BareListError::UnexpectedCommand)),
    }
}

#[cfg(test)]
#[path = "test/list.rs"]
mod tests;
