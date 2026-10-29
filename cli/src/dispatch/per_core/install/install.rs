// Per-core install adapter — normalize each core-specific CLI schema.
// Adapter install theo core — chuẩn hóa schema CLI riêng của từng core.
use crate::commands::definitions::commands::install::InstallCommands;
use crate::dispatch::install_mapping::{InstallRequest, InstallTarget, to_core_command};
use crate::dispatch::types::CoreCommand;

#[allow(unreachable_patterns)] // Keep a None path when feature-gated core variants are absent.
pub(in crate::dispatch) fn map(command: InstallCommands) -> Option<CoreCommand> {
    let (target, request) = match command {
        InstallCommands::InstallWeb {
            packages,
            frozen,
            ignore_scripts,
            allow_scripts,
            prefer_dedupe,
            repair,
            offline,
            compat_runtime,
        } => (
            InstallTarget::Web,
            InstallRequest {
                packages,
                frozen,
                ignore_scripts,
                allow_scripts,
                prefer_dedupe,
                repair,
                offline,
                compat_runtime,
                ..InstallRequest::default()
            },
        ),
        InstallCommands::InstallGame {
            packages,
            frozen,
            compat_runtime,
        } => (
            InstallTarget::Game,
            InstallRequest {
                packages,
                frozen,
                compat_runtime,
                ..InstallRequest::default()
            },
        ),
        InstallCommands::InstallAi {
            packages,
            dry_run,
            compat_runtime,
            frozen,
            offline,
        } => (
            InstallTarget::Ai,
            InstallRequest {
                packages,
                dry_run,
                frozen,
                offline,
                compat_runtime,
                ..InstallRequest::default()
            },
        ),
        InstallCommands::InstallClo {
            packages,
            frozen,
            compat_runtime,
        } => (
            InstallTarget::Clo,
            InstallRequest {
                packages,
                frozen,
                compat_runtime,
                ..InstallRequest::default()
            },
        ),
        InstallCommands::InstallCicd {
            packages,
            compat_runtime,
        } => (
            InstallTarget::Cicd,
            InstallRequest {
                packages,
                compat_runtime,
                ..InstallRequest::default()
            },
        ),
        InstallCommands::InstallIot {
            packages,
            frozen,
            compat_runtime,
        } => (
            InstallTarget::Iot,
            InstallRequest {
                packages,
                frozen,
                compat_runtime,
                ..InstallRequest::default()
            },
        ),
        InstallCommands::InstallApp {
            packages,
            compat_runtime,
            frozen,
            offline,
        } => (
            InstallTarget::App,
            InstallRequest {
                packages,
                frozen,
                offline,
                compat_runtime,
                ..InstallRequest::default()
            },
        ),
        InstallCommands::InstallLibBinary {
            target,
            manifest,
            sign_key,
            trust_key,
        } => {
            // The binary transfer lane is not a dependency operation: it never
            // resolves a manifest, so it maps to its own core command.
            // (Làn chuyển binary không phải thao tác dependency: không resolve
            // manifest, nên map sang core command riêng.)
            return Some(CoreCommand::InstallLibBinary {
                target,
                manifest,
                sign_key,
                trust_key,
            });
        }
        InstallCommands::InstallLib {
            packages,
            compat_runtime,
            frozen,
            offline,
        } => (
            InstallTarget::Lib,
            InstallRequest {
                packages,
                frozen,
                offline,
                compat_runtime,
                ..InstallRequest::default()
            },
        ),
        InstallCommands::InstallHardware {
            packages,
            compat_runtime,
        } => (
            InstallTarget::Hardware,
            InstallRequest {
                packages,
                compat_runtime,
                ..InstallRequest::default()
            },
        ),
        _ => return None,
    };

    Some(to_core_command(target, request))
}

#[cfg(test)]
#[path = "test/install.rs"]
mod tests;
