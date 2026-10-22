// Per-core remove adapter — normalize explicit remove variants into one request.
// Adapter remove theo core — chuẩn hóa variant remove tường minh thành một request.
use crate::commands::definitions::commands::remove::RemoveCommands;
use crate::dispatch::remove_mapping::{RemoveRequest, RemoveTarget, to_core_command};
use crate::dispatch::types::CoreCommand;

#[allow(unreachable_patterns)] // Keep a None path when feature-gated core variants are absent.
pub(in crate::dispatch) fn map(command: RemoveCommands) -> Option<CoreCommand> {
    let (target, packages, install_after_remove, compat_runtime) = match command {
        RemoveCommands::RemoveWeb {
            packages,
            no_install,
            compat_runtime,
        } => (RemoveTarget::Web, packages, !no_install, compat_runtime),
        RemoveCommands::RemoveGame {
            packages,
            compat_runtime,
        } => (RemoveTarget::Game, packages, true, compat_runtime),
        RemoveCommands::RemoveAi {
            packages,
            compat_runtime,
        } => (RemoveTarget::Ai, packages, true, compat_runtime),
        RemoveCommands::RemoveClo {
            packages,
            compat_runtime,
        } => (RemoveTarget::Clo, packages, true, compat_runtime),
        RemoveCommands::RemoveCicd {
            packages,
            compat_runtime,
        } => (RemoveTarget::Cicd, packages, true, compat_runtime),
        RemoveCommands::RemoveIot {
            packages,
            compat_runtime,
        } => (RemoveTarget::Iot, packages, true, compat_runtime),
        RemoveCommands::RemoveApp {
            packages,
            compat_runtime,
        } => (RemoveTarget::App, packages, true, compat_runtime),
        RemoveCommands::RemoveLib {
            packages,
            compat_runtime,
        } => (RemoveTarget::Lib, packages, true, compat_runtime),
        RemoveCommands::RemoveHardware {
            packages,
            compat_runtime,
        } => (RemoveTarget::Hardware, packages, true, compat_runtime),
        _ => return None,
    };

    Some(to_core_command(
        target,
        RemoveRequest {
            packages,
            install_after_remove,
            compat_runtime,
        },
    ))
}

#[cfg(test)]
#[path = "test/remove.rs"]
mod tests;
