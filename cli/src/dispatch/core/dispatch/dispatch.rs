//! Per-core command family routing.
//! Định tuyến họ lệnh theo core.

use anyhow::Result;

use crate::dispatch::types::CoreCommand;

/// Route a core command to its create, install or package-ops handler.
pub async fn dispatch_core(command: CoreCommand) -> Result<()> {
    match command {
        create_command if super::super::create::matches(&create_command) => {
            super::super::create::dispatch(create_command).await
        }
        install_command if super::super::install::matches(&install_command) => {
            super::super::install::dispatch(install_command).await
        }
        package_command => super::super::package_ops::dispatch(package_command).await,
    }
}
