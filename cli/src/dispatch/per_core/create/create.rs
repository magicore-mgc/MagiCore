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
            automation,
        } => Some(CoreCommand::CreateGame {
            framework,
            project_name,
            source,
            automation,
        }),
        CreateCommands::CreateAi {
            framework,
            project_name,
            container,
            source,
            automation,
        } => Some(CoreCommand::CreateAi {
            framework,
            project_name,
            container,
            source,
            automation,
        }),
        CreateCommands::CreateClo {
            framework,
            project_name,
            container,
            source,
            automation,
        } => Some(CoreCommand::CreateClo {
            framework,
            project_name,
            container,
            source,
            automation,
        }),
        CreateCommands::CreateCicd {
            framework,
            project_name,
            source,
            automation,
        } => Some(CoreCommand::CreateCicd {
            framework,
            project_name,
            source,
            automation,
        }),
        CreateCommands::CreateIot {
            framework,
            project_name,
            source,
            automation,
        } => Some(CoreCommand::CreateIot {
            framework,
            project_name,
            source,
            automation,
        }),
        CreateCommands::CreateApp {
            framework,
            project_name,
            container,
            source,
            automation,
        } => Some(CoreCommand::CreateApp {
            framework,
            project_name,
            container,
            source,
            automation,
        }),
        CreateCommands::CreateLib {
            framework,
            project_name,
            container,
            source,
            automation,
        } => Some(CoreCommand::CreateLib {
            framework,
            project_name,
            container,
            source,
            automation,
        }),
        CreateCommands::CreateHardware {
            framework,
            project_name,
            source,
            automation,
        } => Some(CoreCommand::CreateHardware {
            framework,
            project_name,
            source,
            automation,
        }),
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/create.rs"]
mod tests;
