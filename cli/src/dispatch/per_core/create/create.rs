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
            source,
        } => Some(CoreCommand::CreateGame {
            framework,
            project_name,
            source,
        }),
        CreateCommands::CreateAi {
            framework,
            project_name,
            container,
            source,
        } => Some(CoreCommand::CreateAi {
            framework,
            project_name,
            container,
            source,
        }),
        CreateCommands::CreateClo {
            framework,
            project_name,
            container,
            source,
        } => Some(CoreCommand::CreateClo {
            framework,
            project_name,
            container,
            source,
        }),
        CreateCommands::CreateCicd {
            framework,
            project_name,
            source,
        } => Some(CoreCommand::CreateCicd {
            framework,
            project_name,
            source,
        }),
        CreateCommands::CreateIot {
            framework,
            project_name,
            source,
        } => Some(CoreCommand::CreateIot {
            framework,
            project_name,
            source,
        }),
        CreateCommands::CreateApp {
            framework,
            project_name,
            container,
            source,
        } => Some(CoreCommand::CreateApp {
            framework,
            project_name,
            container,
            source,
        }),
        CreateCommands::CreateLib {
            framework,
            project_name,
            container,
            source,
        } => Some(CoreCommand::CreateLib {
            framework,
            project_name,
            container,
            source,
        }),
        CreateCommands::CreateHardware {
            framework,
            project_name,
            source,
        } => Some(CoreCommand::CreateHardware {
            framework,
            project_name,
            source,
        }),
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/create.rs"]
mod tests;
