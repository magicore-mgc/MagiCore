//! Dispatch remove requests to feature-enabled core lanes.
//! Điều phối request remove tới lane core được bật feature.

use crate::commands::core_catalog::is_known_core;
use anyhow::Result;

/// Select one existing core remove lane.
/// Chọn lane remove core hiện có.
pub(in crate::commands::remove) async fn dispatch_remove(
    packages: Vec<String>,
    compat_runtime: Option<String>,
    ecosystem: &str,
) -> Result<()> {
    // Post-remove reinstall follows each lane's CLI default.
    // (Sau remove có reinstall theo default của lane CLI.)
    const MCP_INSTALL_AFTER_REMOVE: bool = false;
    match ecosystem {
        #[cfg(feature = "web")]
        "web" => {
            crate::commands::core::remove::web::remove(
                packages,
                MCP_INSTALL_AFTER_REMOVE,
                compat_runtime,
            )
            .await
        }
        #[cfg(feature = "game")]
        "game" => crate::commands::core::remove::game::remove(packages, compat_runtime).await,
        #[cfg(feature = "ai")]
        "ai" => crate::commands::core::remove::ai::remove(packages, compat_runtime).await,
        #[cfg(feature = "clo")]
        "clo" => crate::commands::core::remove::clo::remove(packages, compat_runtime).await,
        #[cfg(feature = "cicd")]
        "cicd" => crate::commands::core::remove::cicd::remove(packages, compat_runtime).await,
        #[cfg(feature = "iot")]
        "iot" => crate::commands::core::remove::iot::remove(packages, compat_runtime).await,
        #[cfg(feature = "app")]
        "app" => crate::commands::core::remove::app::remove(packages, compat_runtime).await,
        #[cfg(feature = "lib")]
        "lib" => crate::commands::core::remove::library::remove(packages, compat_runtime).await,
        other => Err(if is_known_core(other) {
            crate::error::core_not_in_build(other)
        } else {
            crate::error::unknown_core(other)
        }),
    }
}
