// Per-core list adapter — normalize each core-specific list schema.
// Adapter list theo core — chuẩn hóa schema list riêng của từng core.
use crate::commands::definitions::commands::list::ListCommands;
use crate::dispatch::list_mapping::{ListRequest, ListTarget, to_core_command};
use crate::dispatch::types::CoreCommand;

#[allow(unreachable_patterns)] // Keep a None path when feature-gated core variants are absent.
pub(in crate::dispatch) fn map(command: ListCommands) -> Option<CoreCommand> {
    let (target, request) = match command {
        ListCommands::ListWeb { compat_runtime } => {
            (ListTarget::Web, ListRequest { compat_runtime })
        }
        ListCommands::ListGame { compat_runtime } => {
            (ListTarget::Game, ListRequest { compat_runtime })
        }
        ListCommands::ListAi { compat_runtime } => (ListTarget::Ai, ListRequest { compat_runtime }),
        ListCommands::ListClo { compat_runtime } => {
            (ListTarget::Clo, ListRequest { compat_runtime })
        }
        ListCommands::ListCicd { compat_runtime } => {
            (ListTarget::Cicd, ListRequest { compat_runtime })
        }
        ListCommands::ListIot { compat_runtime } => {
            (ListTarget::Iot, ListRequest { compat_runtime })
        }
        ListCommands::ListApp { compat_runtime } => {
            (ListTarget::App, ListRequest { compat_runtime })
        }
        ListCommands::ListLib { compat_runtime } => {
            (ListTarget::Lib, ListRequest { compat_runtime })
        }
        ListCommands::ListHardware { compat_runtime } => {
            (ListTarget::Hardware, ListRequest { compat_runtime })
        }
        _ => return None,
    };

    Some(to_core_command(target, request))
}

#[cfg(test)]
#[path = "test/list.rs"]
mod tests;
