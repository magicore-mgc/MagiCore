// Per-core create adapter — map explicit scaffold variants to core commands.
// Adapter create theo core — ánh xạ variant scaffold tường minh sang core command.
use crate::commands::definitions::commands::create::CreateCommands;
use crate::dispatch::types::CoreCommand;

#[allow(unreachable_patterns)] // Keep a None path when feature-gated core variants are absent.
pub(in crate::dispatch) fn map(command: CreateCommands) -> Option<CoreCommand> {
    match command {
        CreateCommands::CreateWeb {
            framework,
            project_name,
            flags,
        } => Some(CoreCommand::CreateWeb {
            framework,
            project_name,
            flags,
        }),
        CreateCommands::CreateGame {
            framework,
            project_name,
        } => Some(CoreCommand::CreateGame {
            framework,
            project_name,
        }),
        CreateCommands::CreateAi {
            framework,
            project_name,
            container,
        } => Some(CoreCommand::CreateAi {
            framework,
            project_name,
            container,
        }),
        CreateCommands::CreateClo {
            framework,
            project_name,
            container,
        } => Some(CoreCommand::CreateClo {
            framework,
            project_name,
            container,
        }),
        CreateCommands::CreateCicd {
            framework,
            project_name,
        } => Some(CoreCommand::CreateCicd {
            framework,
            project_name,
        }),
        CreateCommands::CreateIot {
            framework,
            project_name,
        } => Some(CoreCommand::CreateIot {
            framework,
            project_name,
        }),
        CreateCommands::CreateApp {
            framework,
            project_name,
            container,
        } => Some(CoreCommand::CreateApp {
            framework,
            project_name,
            container,
        }),
        CreateCommands::CreateLib {
            framework,
            project_name,
            container,
        } => Some(CoreCommand::CreateLib {
            framework,
            project_name,
            container,
        }),
        CreateCommands::CreateHardware {
            framework,
            project_name,
        } => Some(CoreCommand::CreateHardware {
            framework,
            project_name,
        }),
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/create.rs"]
mod tests;
