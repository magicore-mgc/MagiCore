//! Native App install orchestration.
//! Điều phối cài App native.

use crate::commands::core::app::{language, project_root};
use anyhow::Result;
use std::path::Path;

/// Native flutter install: pubspec → pub.dev → mgc.lock, no toolchain.
/// (Install flutter native, không spawn toolchain.)
pub async fn install_flutter_native(
    root: &std::path::Path,
    packages: Vec<String>,
    dry_run: bool,
    compat_runtime: Option<String>,
    frozen: bool,
    offline: bool,
) -> Result<()> {
    if !packages.is_empty() {
        return Err(super::install_packages_error(&packages));
    }
    let compat = crate::commands::dep_gate::from_dep_flag(compat_runtime.as_deref())?;
    let adapter = mgc_app_adapter::adapter_for(root)
        .ok_or_else(crate::commands::core::app::project_not_detected_error)?;
    crate::commands::dep_gate::gate_native_adapter(
        &crate::commands::dep_gate::DepContext::new(
            "app",
            Some(mgc_app_adapter::AppLanguage::Flutter.ecosystem()),
            None,
            None,
            if offline {
                crate::commands::dep_gate::DepOp::OfflineReinstall
            } else {
                crate::commands::dep_gate::DepOp::Install
            },
        ),
        &adapter,
        &compat,
    )?;
    if dry_run {
        mgc_ui::info(
            "[dry-run] native Flutter dependency install is available; no package manager will be spawned",
        );
        return Ok(());
    }
    let adapter: std::sync::Arc<dyn mgc_types::adapter::PackageAdapter> =
        std::sync::Arc::new(adapter);
    crate::commands::core::shared::install_with_adapter(
        &*adapter,
        root,
        "mgc add",
        frozen,
        mgc_types::adapter::InstallOptions {
            legacy_flat: false,
            offline,
            ..Default::default()
        },
    )
    .await
}

/// `mgc install app` only enters an implemented MagiCore-native lane.
pub async fn install(
    packages: Vec<String>,
    dry_run: bool,
    compat_runtime: Option<String>,
    frozen: bool,
    offline: bool,
) -> Result<()> {
    let root = project_root()?;
    let lang = language(&root)?;

    if matches!(
        lang,
        mgc_app_adapter::AppLanguage::Flutter
            | mgc_app_adapter::AppLanguage::Swift
            | mgc_app_adapter::AppLanguage::ReactNative
    ) {
        return install_app_native(
            &root,
            lang,
            packages,
            dry_run,
            compat_runtime,
            frozen,
            offline,
        )
        .await;
    }

    Err(crate::commands::core::app::native_lifecycle_error(
        lang.ecosystem(),
        "install",
    ))
}

async fn install_app_native(
    root: &Path,
    lang: mgc_app_adapter::AppLanguage,
    packages: Vec<String>,
    dry_run: bool,
    compat_runtime: Option<String>,
    frozen: bool,
    offline: bool,
) -> Result<()> {
    if !packages.is_empty() && lang != mgc_app_adapter::AppLanguage::Flutter {
        return Err(crate::commands::core::app::native_lifecycle_error(
            lang.ecosystem(),
            "install package arguments; use the native add operation",
        ));
    }
    if lang == mgc_app_adapter::AppLanguage::Flutter {
        return install_flutter_native(root, packages, dry_run, compat_runtime, frozen, offline)
            .await;
    }
    let compat = crate::commands::dep_gate::from_dep_flag(compat_runtime.as_deref())?;
    let adapter = mgc_app_adapter::adapter_for(root)
        .ok_or_else(crate::commands::core::app::project_not_detected_error)?;
    crate::commands::dep_gate::gate_native_adapter(
        &crate::commands::dep_gate::DepContext::new(
            "app",
            Some(lang.ecosystem()),
            None,
            None,
            if offline {
                crate::commands::dep_gate::DepOp::OfflineReinstall
            } else {
                crate::commands::dep_gate::DepOp::Install
            },
        ),
        &adapter,
        &compat,
    )?;
    if dry_run {
        mgc_ui::info(&format!(
            "[dry-run] native app dependency install is available for {}; no package manager will be spawned",
            lang.as_str()
        ));
        return Ok(());
    }
    crate::commands::core::shared::install_with_adapter(
        &adapter,
        root,
        "mgc install app",
        frozen,
        mgc_types::adapter::InstallOptions {
            legacy_flat: false,
            offline,
            ..Default::default()
        },
    )
    .await
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
