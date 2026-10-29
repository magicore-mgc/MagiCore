// Audit-strict policy — allow the flag only on commands that enforce it.
// Owner này chỉ cho phép cờ audit-strict ở lệnh có áp dụng thực tế.
use super::super::command_name::command_name;
use super::super::error::EnginePolicyError;
use crate::commands::definitions::commands::{
    AddCommands, BareCommands, Commands, CommonCommands, EngineCommands, InstallCommands,
};
use anyhow::Result;

pub(in crate::dispatch) fn reject_unsupported_audit_strict(command: &Commands) -> Result<()> {
    // `--audit-strict` is meaningful exactly where the codebase consults
    // strictness: the audit command (`StrictMode`), the verify chain
    // (`run_strict`), and install/add flows (`prepare_install_execution`
    // → `enforce_audit_strict_policy`). Everywhere else the flag would be
    // silently ignored — refuse loudly with the supported set instead.
    // (Chỉ cho phép cờ ở lệnh có dùng strict; nơi khác từ chối rõ ràng.)
    if matches!(
        command,
        Commands::Common(CommonCommands::Audit { .. })
            | Commands::Engine(EngineCommands::Verify)
            | Commands::Bare(BareCommands::Install { .. })
            | Commands::Bare(BareCommands::Add { .. })
            | Commands::Install(InstallCommands::InstallWeb { .. })
            | Commands::Install(InstallCommands::InstallGame { .. })
            | Commands::Install(InstallCommands::InstallAi { .. })
            | Commands::Install(InstallCommands::InstallClo { .. })
            | Commands::Install(InstallCommands::InstallCicd { .. })
            | Commands::Install(InstallCommands::InstallIot { .. })
            | Commands::Install(InstallCommands::InstallApp { .. })
            | Commands::Install(InstallCommands::InstallLibBinary { .. })
            | Commands::Install(InstallCommands::InstallLib { .. })
            | Commands::Install(InstallCommands::InstallHardware { .. })
            | Commands::Add(AddCommands::AddWeb { .. })
            | Commands::Add(AddCommands::AddGame { .. })
            | Commands::Add(AddCommands::AddAi { .. })
            | Commands::Add(AddCommands::AddClo { .. })
            | Commands::Add(AddCommands::AddCicd { .. })
            | Commands::Add(AddCommands::AddIot { .. })
            | Commands::Add(AddCommands::AddApp { .. })
            | Commands::Add(AddCommands::AddLib { .. })
            | Commands::Add(AddCommands::AddHardware { .. })
    ) {
        return Ok(());
    }
    Err(EnginePolicyError::AuditStrictUnsupported {
        command: command_name(command),
    }
    .into_error())
}
