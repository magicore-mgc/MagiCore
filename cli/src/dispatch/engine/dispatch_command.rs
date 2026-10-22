// Dispatch adapter — call the typed shared/core handler after command mapping.
// Adapter này gọi handler chung hoặc handler core sau bước ánh xạ.
use crate::Commands;
use crate::dispatch::types::DispatchCommand;
use anyhow::Result;

pub(super) async fn dispatch_command(
    command: Commands,
    core: Option<&str>,
    recursive: bool,
) -> Result<()> {
    match crate::dispatch::per_core::command_to_dispatch(command, core)? {
        DispatchCommand::Common(cmd) => {
            super::super::common::dispatch_common(cmd, core, recursive).await
        }
        DispatchCommand::Core(cmd) => super::super::core::dispatch_core(cmd).await,
    }
}
