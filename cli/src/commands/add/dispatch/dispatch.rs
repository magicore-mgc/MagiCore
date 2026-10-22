//! Dispatch add requests to feature-enabled core lanes.
//! Điều phối request add tới lane core được bật feature.

use super::super::types::AddRequest;
use crate::commands::core_catalog::is_known_core;
use anyhow::Result;

/// Select one existing core add lane without changing its arguments.
/// Chọn lane add core hiện có và giữ nguyên đối số.
pub(in crate::commands::add) async fn dispatch_add(
    request: AddRequest,
    ecosystem: &str,
) -> Result<()> {
    let AddRequest {
        packages,
        version,
        dev,
        exact,
        optional,
        peer,
        no_save,
        global,
        compat_runtime,
    } = request;
    // Post-add install follows each lane's CLI default (web honors the
    // flag below; lib/game/ai/clo/iot/app lanes install like their CLI
    // counterparts). MCP previously never installed — unification with
    // the CLI is intentional: one behavior, one gate, no shadow path.
    // (Sau add có install theo default của lane CLI — thống nhất.)
    const MCP_INSTALL_AFTER_ADD: bool = false;
    match ecosystem {
        #[cfg(feature = "web")]
        "web" => {
            crate::commands::core::add::web::add(
                packages,
                version,
                dev,
                exact,
                optional,
                peer,
                no_save,
                MCP_INSTALL_AFTER_ADD,
                global,
                compat_runtime,
            )
            .await
        }
        #[cfg(feature = "game")]
        "game" => {
            crate::commands::core::add::game::add(
                packages,
                version,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
            )
            .await
        }
        #[cfg(feature = "ai")]
        "ai" => {
            crate::commands::core::add::ai::add(
                packages,
                version,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
            )
            .await
        }
        #[cfg(feature = "clo")]
        "clo" => {
            crate::commands::core::add::clo::add(
                packages,
                version,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
            )
            .await
        }
        #[cfg(feature = "cicd")]
        "cicd" => {
            crate::commands::core::add::cicd::add(
                packages,
                version,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
            )
            .await
        }
        #[cfg(feature = "iot")]
        "iot" => {
            crate::commands::core::add::iot::add(
                packages,
                version,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
            )
            .await
        }
        #[cfg(feature = "app")]
        "app" => {
            crate::commands::core::add::app::add(
                packages,
                version,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
            )
            .await
        }
        #[cfg(feature = "lib")]
        "lib" => {
            crate::commands::core::add::library::add(
                packages,
                version,
                dev,
                exact,
                optional,
                peer,
                no_save,
                global,
                compat_runtime,
            )
            .await
        }
        #[cfg(feature = "hardware")]
        "hardware" => {
            crate::commands::core::add::hardware::add(packages, compat_runtime, version).await
        }
        other => Err(if is_known_core(other) {
            // Known core whose lane is compiled out of THIS build —
            // fail closed, never fall through to a direct adapter call.
            // (Core quen nhưng lane không có trong build này.)
            crate::error::core_not_in_build(other)
        } else {
            crate::error::unknown_core(other)
        }),
    }
}
