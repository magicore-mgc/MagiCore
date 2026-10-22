// Recursive flag policy — centralize the supported command set and refusal messages.
// Owner này tập trung danh sách hỗ trợ và thông báo từ chối recursive/filter.
use super::super::command_name::command_name;
use super::super::error::EnginePolicyError;
use crate::commands::definitions::commands::{
    AddCommands, BareCommands, Commands, CommonCommands, EngineCommands, InstallCommands,
    ListCommands, RemoveCommands, UpdateCommands,
};
use anyhow::Result;

pub(in crate::dispatch::engine) fn recursive_supported(command: &Commands) -> bool {
    // T4: mở rộng danh sách lệnh hỗ trợ --recursive (pnpm -r parity)
    // Thêm: Build, Run, Audit, Outdated, Dev (core-aware per workspace)
    matches!(
        command,
        Commands::Common(CommonCommands::Publish { .. })
            | Commands::Bare(BareCommands::Install { .. })
            | Commands::Bare(BareCommands::Add { .. })
            | Commands::Bare(BareCommands::Remove { .. })
            | Commands::Bare(BareCommands::Update { .. })
            | Commands::Bare(BareCommands::List { .. })
            | Commands::Engine(EngineCommands::Build { .. })
            | Commands::Engine(EngineCommands::Run { .. })
            | Commands::Engine(EngineCommands::Test { .. })
            | Commands::Engine(EngineCommands::Optimizer { .. })
            | Commands::Common(CommonCommands::Audit { .. })
            | Commands::Common(CommonCommands::Outdated { .. })
            | Commands::Engine(EngineCommands::Dev { .. })
            | Commands::Install(InstallCommands::InstallWeb { .. })
            | Commands::Install(InstallCommands::InstallGame { .. })
            | Commands::Install(InstallCommands::InstallAi { .. })
            | Commands::Install(InstallCommands::InstallClo { .. })
            | Commands::Install(InstallCommands::InstallCicd { .. })
            | Commands::Install(InstallCommands::InstallIot { .. })
            | Commands::Install(InstallCommands::InstallApp { .. })
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
            | Commands::Remove(RemoveCommands::RemoveWeb { .. })
            | Commands::Remove(RemoveCommands::RemoveGame { .. })
            | Commands::Remove(RemoveCommands::RemoveAi { .. })
            | Commands::Remove(RemoveCommands::RemoveClo { .. })
            | Commands::Remove(RemoveCommands::RemoveCicd { .. })
            | Commands::Remove(RemoveCommands::RemoveIot { .. })
            | Commands::Remove(RemoveCommands::RemoveApp { .. })
            | Commands::Remove(RemoveCommands::RemoveLib { .. })
            | Commands::Remove(RemoveCommands::RemoveHardware { .. })
            | Commands::Update(UpdateCommands::UpdateWeb { .. })
            | Commands::Update(UpdateCommands::UpdateGame { .. })
            | Commands::Update(UpdateCommands::UpdateAi { .. })
            | Commands::Update(UpdateCommands::UpdateClo { .. })
            | Commands::Update(UpdateCommands::UpdateCicd { .. })
            | Commands::Update(UpdateCommands::UpdateIot { .. })
            | Commands::Update(UpdateCommands::UpdateApp { .. })
            | Commands::Update(UpdateCommands::UpdateLib { .. })
            | Commands::Update(UpdateCommands::UpdateHardware { .. })
            | Commands::List(ListCommands::ListWeb { .. })
            | Commands::List(ListCommands::ListGame { .. })
            | Commands::List(ListCommands::ListAi { .. })
            | Commands::List(ListCommands::ListClo { .. })
            | Commands::List(ListCommands::ListCicd { .. })
            | Commands::List(ListCommands::ListIot { .. })
            | Commands::List(ListCommands::ListApp { .. })
            | Commands::List(ListCommands::ListLib { .. })
            | Commands::List(ListCommands::ListHardware { .. })
    )
}

pub(in crate::dispatch::engine) fn reject_unsupported_recursive(
    command: Option<&Commands>,
) -> Result<()> {
    let Some(command) = command else {
        return Err(EnginePolicyError::RecursiveNotAvailable.into_error());
    };

    Err(EnginePolicyError::RecursiveUnsupported {
        command: command_name(command),
    }
    .into_error())
}

pub(in crate::dispatch::engine) fn reject_unsupported_filter(command: &Commands) -> Result<()> {
    Err(EnginePolicyError::FilterUnsupported {
        command: command_name(command),
    }
    .into_error())
}

pub(in crate::dispatch::engine) fn reject_filter_without_recursive() -> Result<()> {
    Err(EnginePolicyError::FilterRequiresRecursive.into_error())
}
