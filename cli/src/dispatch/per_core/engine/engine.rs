// Engine CLI adapter — map engine commands into shared dispatch values.
// Adapter CLI engine — ánh xạ lệnh engine sang giá trị dispatch dùng chung.
use crate::commands::definitions::commands::engine::EngineCommands;
use crate::dispatch::types::CommonCommand;

pub(in crate::dispatch) fn engine_to_common(command: EngineCommands) -> Option<CommonCommand> {
    match command {
        EngineCommands::Dev {
            host,
            port,
            clear,
            compat_runtime,
            compat_runner,
        } => Some(CommonCommand::Dev {
            host,
            port,
            clear,
            compat_runtime,
            compat_runner,
        }),
        EngineCommands::Run {
            script,
            args,
            compat_runtime,
            compat_runner,
        } => Some(CommonCommand::Run {
            script,
            args,
            compat_runtime,
            compat_runner,
        }),
        EngineCommands::Test {
            args,
            compat_runtime,
            compat_runner,
        } => Some(CommonCommand::Test {
            args,
            compat_runtime,
            compat_runner,
        }),
        EngineCommands::Optimizer { force } => Some(CommonCommand::Optimizer { force }),
        EngineCommands::Build {
            target,
            compat_runtime,
            compat_runner,
            engine,
            allow_output_warnings,
        } => Some(CommonCommand::Build {
            target,
            compat_runtime,
            compat_runner,
            engine,
            allow_output_warnings,
        }),
        EngineCommands::Flash { board, skip_build } => {
            Some(CommonCommand::Flash { board, skip_build })
        }
        EngineCommands::VerifySbom {
            artifact,
            trust_key,
        } => Some(CommonCommand::VerifySbom {
            artifact,
            trust_key,
        }),
        EngineCommands::Deploy { run } => Some(CommonCommand::Deploy { run }),
        EngineCommands::CiGenerate => Some(CommonCommand::CiGenerate),
        EngineCommands::Verify => Some(CommonCommand::Verify),
        EngineCommands::Start => Some(CommonCommand::Start),
        EngineCommands::Exec {
            command,
            args,
            compat_runner,
        } => Some(CommonCommand::Exec {
            command,
            args,
            compat_runner,
        }),
        EngineCommands::Dlx {
            package,
            bin,
            allow_scripts,
            args,
        } => Some(CommonCommand::Dlx {
            package,
            bin,
            allow_scripts,
            args,
        }),
        EngineCommands::Tool { cmd } => Some(CommonCommand::Tool { cmd }),
        EngineCommands::Cache {
            action,
            target,
            yes,
            dry_run,
            ci,
        } => Some(CommonCommand::Cache {
            action,
            target,
            yes,
            dry_run,
            ci,
        }),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
