// Per-core update adapter — normalize explicit variants into one request.
// Adapter update theo core — chuẩn hóa variant tường minh thành một request.
use crate::commands::definitions::commands::update::UpdateCommands;
use crate::dispatch::types::CoreCommand;
use crate::dispatch::update_mapping::{UpdateRequest, UpdateTarget, to_core_command};

#[allow(unreachable_patterns)] // Keep a None path when feature-gated core variants are absent.
pub(in crate::dispatch) fn map(command: UpdateCommands) -> Option<CoreCommand> {
    let (target, packages, install, compat_runtime) = match command {
        UpdateCommands::UpdateWeb {
            packages,
            install,
            compat_runtime,
        } => (UpdateTarget::Web, packages, install, compat_runtime),
        UpdateCommands::UpdateGame {
            packages,
            install,
            compat_runtime,
        } => (UpdateTarget::Game, packages, install, compat_runtime),
        UpdateCommands::UpdateAi {
            packages,
            install,
            compat_runtime,
        } => (UpdateTarget::Ai, packages, install, compat_runtime),
        UpdateCommands::UpdateClo {
            packages,
            install,
            compat_runtime,
        } => (UpdateTarget::Clo, packages, install, compat_runtime),
        UpdateCommands::UpdateCicd {
            packages,
            install,
            compat_runtime,
        } => (UpdateTarget::Cicd, packages, install, compat_runtime),
        UpdateCommands::UpdateIot {
            packages,
            install,
            compat_runtime,
        } => (UpdateTarget::Iot, packages, install, compat_runtime),
        UpdateCommands::UpdateApp {
            packages,
            install,
            compat_runtime,
        } => (UpdateTarget::App, packages, install, compat_runtime),
        UpdateCommands::UpdateLib {
            packages,
            install,
            compat_runtime,
        } => (UpdateTarget::Lib, packages, install, compat_runtime),
        UpdateCommands::UpdateHardware {
            packages,
            install,
            compat_runtime,
        } => (UpdateTarget::Hardware, packages, install, compat_runtime),
        _ => return None,
    };

    Some(to_core_command(
        target,
        UpdateRequest {
            packages,
            install,
            compat_runtime,
        },
    ))
}

#[cfg(test)]
#[path = "test/update.rs"]
mod tests;
