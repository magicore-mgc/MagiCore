// Bare install adapter — convert the detected core and CLI flags to a shared request.
// Adapter install bare — đổi core phát hiện và cờ CLI thành request dùng chung.
use crate::commands::definitions::commands::bare::BareCommands;
use crate::dispatch::install_mapping::{InstallRequest, InstallTarget, to_core_command};
use crate::dispatch::types::CoreCommand;

use super::error::BareInstallError;
use crate::dispatch::bare::ecosystem::require_ecosystem;
use crate::dispatch::bare::error::unknown_core;

pub(in crate::dispatch) fn map(
    command: BareCommands,
    ecosystem: Option<&str>,
) -> anyhow::Result<CoreCommand> {
    match command {
        BareCommands::Install {
            packages,
            frozen,
            ignore_scripts,
            allow_scripts,
            prefer_dedupe,
            repair,
            dry_run,
            offline,
            compat_runtime,
        } => {
            let core = require_ecosystem("install", ecosystem)?;
            let target = InstallTarget::from_core_name(core).ok_or_else(|| unknown_core(core))?;
            let request = InstallRequest {
                packages,
                dry_run,
                frozen,
                ignore_scripts,
                allow_scripts,
                prefer_dedupe,
                repair,
                offline,
                compat_runtime,
            };

            Ok(to_core_command(target, request))
        }
        _ => Err(anyhow::Error::new(BareInstallError::UnexpectedCommand)),
    }
}

#[cfg(test)]
#[path = "test/install.rs"]
mod tests;
