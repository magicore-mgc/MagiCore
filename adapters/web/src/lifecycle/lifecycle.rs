//! Coordinate guarded package lifecycle hooks while preserving their order.
//! Điều phối các hook package đã được bảo vệ và giữ đúng thứ tự thực thi.

use super::error::LifecycleError;
use super::identity_guard::ensure_project_identity;
#[cfg(test)]
use super::manifest::load_package_scripts;
use crate::lifecycle::types::PackageScripts;
use mgc_types::error::MgResult;
use std::path::Path;

/// Lifecycle executor available to the guarded install flow.
/// Bộ chạy lifecycle chỉ được gọi từ luồng install đã kiểm tra trust.
///
/// External callers cannot bypass that approval gate:
/// Caller bên ngoài không thể bỏ qua cổng phê duyệt đó:
///
/// ```compile_fail
/// use mgc_web_adapter::lifecycle::LifecycleRunner;
/// use std::path::Path;
/// LifecycleRunner::run_scripts(Path::new("."), Path::new(".")).unwrap();
/// ```
pub struct LifecycleRunner;

impl LifecycleRunner {
    #[cfg(test)]
    pub(crate) fn run_scripts(pkg_dir: &Path, project_root: &Path) -> MgResult<()> {
        let scripts = load_package_scripts(pkg_dir)?;
        Self::run_scripts_with_snapshot(pkg_dir, project_root, scripts)
    }

    pub(crate) fn run_scripts_with_snapshot(
        pkg_dir: &Path,
        project_root: &Path,
        scripts: PackageScripts,
    ) -> MgResult<()> {
        let owner_before = mgc_config::project::ProjectConfig::detect_core(project_root)
            .map_err(|source| LifecycleError::IdentityPreflight { source })?;
        let identity_snapshot = mgc_config::project::snapshot_project_identity(project_root)
            .map_err(|source| LifecycleError::IdentitySnapshot { source })?;

        // Deterministic transaction-test failpoint; it rejects before running hooks.
        // Móc lỗi transaction test có tính xác định; từ chối trước khi chạy hook.
        if let Ok(filter) = std::env::var("MGC_LIFECYCLE_FAIL_PACKAGES") {
            let package = pkg_dir
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            if filter
                .split(',')
                .map(str::trim)
                .any(|entry| entry == "*" || entry == package)
            {
                return Err(LifecycleError::InjectedFailure { package }.into());
            }
        }

        if let Some(script) = scripts.preinstall {
            super::script_execution::run_script(pkg_dir, project_root, "preinstall", &script)?;
            ensure_project_identity(project_root, &owner_before, &identity_snapshot)?;
        }
        if let Some(script) = scripts.install {
            super::script_execution::run_script(pkg_dir, project_root, "install", &script)?;
            ensure_project_identity(project_root, &owner_before, &identity_snapshot)?;
        }
        if let Some(script) = scripts.postinstall {
            super::script_execution::run_script(pkg_dir, project_root, "postinstall", &script)?;
            ensure_project_identity(project_root, &owner_before, &identity_snapshot)?;
        }
        ensure_project_identity(project_root, &owner_before, &identity_snapshot)?;
        Ok(())
    }
}
