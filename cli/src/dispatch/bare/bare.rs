// Bare package coordinator — resolve the owner core, validate flags, then map the verb.
// Bộ điều phối chọn core, kiểm tra cờ rồi giao thao tác cho mapper riêng.
use crate::commands::definitions::commands::BareCommands;
use crate::dispatch::types::{DispatchCommand, detect_ecosystem};

use super::{add, install, list, remove, update, validate};

pub fn bare_core_command(
    command: BareCommands,
    ecosystem: Option<String>,
) -> anyhow::Result<DispatchCommand> {
    let ecosystem = match ecosystem {
        Some(core) => Some(core),
        None => detect_ecosystem()?,
    };
    validate::validate_bare_request(&command, ecosystem.as_deref())?;

    let core_command = match command {
        value @ BareCommands::Install { .. } => install::map(value, ecosystem.as_deref())?,
        value @ BareCommands::Add { .. } => add::map(value, ecosystem.as_deref())?,
        value @ BareCommands::Remove { .. } => remove::map(value, ecosystem.as_deref())?,
        value @ BareCommands::Update { .. } => update::map(value, ecosystem.as_deref())?,
        value @ BareCommands::List { .. } => list::map(value, ecosystem.as_deref())?,
        _ => unreachable!("Unhandled command"),
    };
    Ok(DispatchCommand::Core(core_command))
}
