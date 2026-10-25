// Bare common adapter — map shared CLI commands and preserve core fallback.
// Adapter bare common — ánh xạ lệnh CLI dùng chung và giữ fallback theo core.
use crate::commands::definitions::commands::bare::BareCommands;
use crate::dispatch::types::CommonCommand;

pub(in crate::dispatch) fn bare_common_to_common(command: BareCommands) -> Option<CommonCommand> {
    match command {
        BareCommands::Link { package } => Some(CommonCommand::Link { package }),
        BareCommands::Unlink { package } => Some(CommonCommand::Unlink { package }),
        BareCommands::Why { package } => Some(CommonCommand::Why { package }),
        BareCommands::Store { cmd } => Some(CommonCommand::Store { cmd }),
        BareCommands::Bench { args } => Some(CommonCommand::Bench { args }),
        BareCommands::Network { cmd } => Some(CommonCommand::Network { cmd }),
        BareCommands::Doctor { cmd } => Some(CommonCommand::Doctor { cmd }),
        BareCommands::Trust { cmd } => Some(CommonCommand::Trust { cmd }),
        BareCommands::Hooks { cmd } => Some(CommonCommand::Hooks { cmd }),
        BareCommands::Docs { output } => Some(CommonCommand::Docs { output }),
        BareCommands::Completion { shell } => Some(CommonCommand::Completion { shell }),
        BareCommands::Telemetry { cmd } => Some(CommonCommand::Telemetry { cmd }),
        BareCommands::Workspace { cmd } => Some(CommonCommand::Workspace { cmd }),
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
