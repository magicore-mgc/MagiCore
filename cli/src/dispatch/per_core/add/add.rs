// Per-core add adapter — normalize each core-specific command schema.
// Adapter add theo core — chuẩn hóa schema lệnh riêng của từng core.
use crate::commands::definitions::commands::add::AddCommands;
use crate::dispatch::add_mapping::{AddRequest, AddTarget, to_core_command};
use crate::dispatch::types::CoreCommand;

#[allow(unreachable_patterns)] // Keep a None path when feature-gated core variants are absent.
pub(in crate::dispatch) fn map(command: AddCommands) -> Option<CoreCommand> {
    let (target, request) = match command {
        AddCommands::AddWeb {
            packages,
            dev,
            exact,
            optional,
            peer,
            no_save,
            no_install,
            global,
            compat_runtime,
            version,
        } => (
            AddTarget::Web,
            AddRequest {
                packages,
                dev,
                exact,
                optional,
                peer,
                no_save,
                install: !no_install,
                global,
                compat_runtime,
                version,
            },
        ),
        AddCommands::AddGame {
            packages,
            dev,
            exact,
            optional,
            peer,
            no_save,
            global,
            compat_runtime,
            version,
        } => (
            AddTarget::Game,
            AddRequest {
                packages,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
                version,
                ..AddRequest::default()
            },
        ),
        AddCommands::AddAi {
            packages,
            dev,
            exact,
            optional,
            peer,
            no_save,
            global,
            compat_runtime,
            version,
        } => (
            AddTarget::Ai,
            AddRequest {
                packages,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
                version,
                ..AddRequest::default()
            },
        ),
        AddCommands::AddClo {
            packages,
            dev,
            exact,
            optional,
            peer,
            no_save,
            global,
            compat_runtime,
            version,
        } => (
            AddTarget::Clo,
            AddRequest {
                packages,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
                version,
                ..AddRequest::default()
            },
        ),
        AddCommands::AddCicd {
            packages,
            dev,
            exact,
            optional,
            peer,
            no_save,
            global,
            compat_runtime,
            version,
        } => (
            AddTarget::Cicd,
            AddRequest {
                packages,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
                version,
                ..AddRequest::default()
            },
        ),
        AddCommands::AddIot {
            packages,
            dev,
            exact,
            optional,
            peer,
            no_save,
            global,
            compat_runtime,
            version,
        } => (
            AddTarget::Iot,
            AddRequest {
                packages,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
                version,
                ..AddRequest::default()
            },
        ),
        AddCommands::AddApp {
            packages,
            dev,
            exact,
            optional,
            peer,
            no_save,
            global,
            compat_runtime,
            version,
        } => (
            AddTarget::App,
            AddRequest {
                packages,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
                version,
                ..AddRequest::default()
            },
        ),
        AddCommands::AddLib {
            packages,
            dev,
            exact,
            optional,
            peer,
            no_save,
            global,
            compat_runtime,
            version,
        } => (
            AddTarget::Lib,
            AddRequest {
                packages,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
                version,
                ..AddRequest::default()
            },
        ),
        AddCommands::AddHardware {
            packages,
            compat_runtime,
            version,
        } => (
            AddTarget::Hardware,
            AddRequest {
                packages,
                compat_runtime,
                version,
                ..AddRequest::default()
            },
        ),
        _ => return None,
    };

    Some(to_core_command(target, request))
}

#[cfg(test)]
#[path = "test/add.rs"]
mod tests;
