// Bare add adapter — convert detected core and CLI flags into the shared contract.
// Adapter add bare — chuyển core phát hiện và cờ CLI thành hợp đồng dùng chung.
use crate::commands::definitions::commands::bare::BareCommands;
use crate::dispatch::add_mapping::{AddRequest, AddTarget, to_core_command};
use crate::dispatch::types::CoreCommand;

use super::error::BareAddError;
use crate::dispatch::bare::ecosystem::require_ecosystem;
use crate::dispatch::bare::error::unknown_core;

pub(in crate::dispatch) fn map(
    command: BareCommands,
    ecosystem: Option<&str>,
) -> anyhow::Result<CoreCommand> {
    match command {
        BareCommands::Add {
            packages,
            version,
            dev,
            global,
            exact,
            optional,
            peer,
            no_save,
            no_install,
            compat_runtime,
        } => {
            let core = require_ecosystem("add", ecosystem)?;
            let target = AddTarget::from_core_name(core).ok_or_else(|| unknown_core(core))?;
            let request = AddRequest {
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
            };

            Ok(to_core_command(target, request))
        }
        _ => Err(anyhow::Error::new(BareAddError::UnexpectedCommand)),
    }
}

#[cfg(test)]
#[path = "test/add.rs"]
mod tests;
