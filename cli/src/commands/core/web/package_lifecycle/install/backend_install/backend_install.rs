//! Routes non-JS Web backends through the native language adapter.
//! Định tuyến backend Web non-JS qua adapter ngôn ngữ native.

use anyhow::Result;
use std::path::Path;

use super::super::options::WebInstallOptions;
use crate::commands::core::shared;
#[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
use crate::commands::core::web::implementation::project_context::adapter_selection::{
    web_backend_adapter_for_operation, web_backend_language,
};

/// Installs a recognized non-JS backend and reports whether it handled the request.
/// Cài backend non-JS đã nhận diện và báo liệu request đã được xử lý chưa.
pub(in crate::commands::core::web::implementation::package_lifecycle::install) async fn install_if_applicable(
    root: &Path,
    options: &WebInstallOptions,
) -> Result<bool> {
    #[cfg(any(feature = "lib", feature = "web-backend-runtime"))]
    if let Some(backend) = web_backend_language(root) {
        let language = backend.ecosystem();
        if options.ignore_scripts {
            return Err(crate::error::web_backend_flag_unsupported(
                "--ignore-scripts",
                language,
            ));
        }
        if options.allow_scripts {
            return Err(crate::error::web_backend_flag_unsupported(
                "--allow-scripts",
                language,
            ));
        }
        if options.prefer_dedupe {
            return Err(crate::error::web_backend_flag_unsupported(
                "--prefer-dedupe",
                language,
            ));
        }
        if options.repair {
            return Err(crate::error::web_backend_flag_unsupported(
                "--repair", language,
            ));
        }
        if !options.packages.is_empty() {
            return Err(crate::error::install_lib_packages_use_add(
                &options.packages,
            ));
        }
        let operation = if options.offline {
            crate::commands::dep_gate::DepOp::OfflineReinstall
        } else {
            crate::commands::dep_gate::DepOp::Install
        };
        let adapter = web_backend_adapter_for_operation(
            root,
            backend,
            operation,
            options.compat_runtime.as_deref(),
        )?;
        shared::install_with_adapter(
            &*adapter,
            root,
            "mgc add",
            options.frozen || options.offline,
            mgc_types::adapter::InstallOptions {
                legacy_flat: false,
                offline: options.offline,
                ..Default::default()
            },
        )
        .await?;
        return Ok(true);
    }

    let _ = (root, options);
    Ok(false)
}
