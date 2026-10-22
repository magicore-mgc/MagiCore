// Typed dispatcher coordinator — routes each command group to its owner.
// Bộ điều phối này gửi từng nhóm lệnh đến mapper phụ trách.
use crate::commands::definitions::commands::Commands;
use crate::dispatch::bare;
use crate::dispatch::types::DispatchCommand;

use super::{add, bare_common, common, create, engine, install, list, remove, update};

pub fn command_to_dispatch(
    command: Commands,
    core: Option<&str>,
) -> anyhow::Result<DispatchCommand> {
    let common = match command.clone() {
        Commands::Common(value) => common::common_to_dispatch(value),
        Commands::Engine(value) => engine::engine_to_common(value),
        Commands::Bare(value) => bare_common::bare_common_to_common(value),
        _ => None,
    };
    if let Some(value) = common {
        return Ok(DispatchCommand::Common(value));
    }

    let explicit_core = match command.clone() {
        Commands::Create(value) => create::map(value),
        Commands::Install(value) => install::map(value),
        Commands::Add(value) => add::map(value),
        Commands::Remove(value) => remove::map(value),
        Commands::List(value) => list::map(value),
        Commands::Update(value) => update::map(value),
        _ => None,
    };
    if let Some(value) = explicit_core {
        return Ok(DispatchCommand::Core(value));
    }

    let ecosystem = match core {
        Some(core) => Some(core.to_string()),
        None => crate::dispatch::types::detect_ecosystem()?,
    };
    match command {
        Commands::Bare(value) => bare::bare_core_command(value, ecosystem),
        _ => unreachable!("Unhandled command"),
    }
}
