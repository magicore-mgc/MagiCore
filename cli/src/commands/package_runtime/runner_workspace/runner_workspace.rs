//! Creates the isolated MGC-managed project used by a registry package run.
//! Tạo project cô lập do MGC quản lý cho lượt chạy package registry.

use super::super::work_root::PackageRunPurpose;
use anyhow::Result;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub(in crate::commands::package_runtime) struct RunnerWorkspace {
    // Keep the temporary project alive until the caller finishes execution.
    // Giữ project tạm tới khi caller chạy xong package.
    pub(in crate::commands::package_runtime) _directory: TempDir,
    pub(in crate::commands::package_runtime) project_root: PathBuf,
    pub(in crate::commands::package_runtime) canonical_project: PathBuf,
}

pub(in crate::commands::package_runtime) fn prepare_runner_workspace(
    cwd: &Path,
    package: &str,
    selector: &str,
    purpose: PackageRunPurpose,
) -> Result<RunnerWorkspace> {
    let project_config = super::super::install_policy::registry_project_config(cwd)?;
    let data_root = mgc_platform::paths::mgc_data_dir();
    prepare_runner_workspace_with_config(
        &data_root,
        cwd,
        package,
        selector,
        purpose,
        project_config,
    )
}

#[cfg(test)]
fn prepare_runner_workspace_at(
    data_root: &Path,
    cwd: &Path,
    package: &str,
    selector: &str,
    purpose: PackageRunPurpose,
) -> Result<RunnerWorkspace> {
    let project_config = super::super::install_policy::registry_project_config(cwd)?;
    prepare_runner_workspace_with_config(data_root, cwd, package, selector, purpose, project_config)
}

fn prepare_runner_workspace_with_config(
    data_root: &Path,
    cwd: &Path,
    package: &str,
    selector: &str,
    purpose: PackageRunPurpose,
    project_config: mgc_config::project::ProjectConfig,
) -> Result<RunnerWorkspace> {
    let work_root = super::super::work_root::work_root_at(data_root, purpose)?;
    let directory = tempfile::Builder::new()
        .prefix(purpose.work_prefix())
        .tempdir_in(&work_root)?;
    let project_root = directory.path().to_path_buf();
    let canonical_project = project_root.canonicalize()?;
    if !canonical_project.starts_with(&work_root) {
        return Err(super::super::work_root::registry_package_cache_path_unsafe(
            &project_root,
        ));
    }

    let pkg_json = serde_json::json!({
        "name": super::super::install_policy::PACKAGE_RUNNER_PROJECT_NAME,
        "version": super::super::install_policy::PACKAGE_RUNNER_PROJECT_VERSION,
        "dependencies": {package: selector}
    });
    std::fs::write(
        project_root.join("package.json"),
        serde_json::to_string_pretty(&pkg_json)?,
    )?;
    std::fs::write(
        project_root.join("mgc.toml"),
        toml::to_string(&project_config)?,
    )?;
    // Validate the exact inherited snapshot before launching the installer.
    // Kiểm schema snapshot kế thừa trước khi chạy installer.
    mgc_config::project::lock_schema_is_v4(&project_root)?;
    // Preserve explicit project registry/auth configuration without logging it.
    // Giữ cấu hình registry/auth tường minh, không ghi credential vào log.
    if let Some(npmrc) =
        mgc_config::project::read_regular_project_text(&cwd.join(".npmrc"), ".npmrc")?
    {
        std::fs::write(project_root.join(".npmrc"), npmrc)?;
    }

    Ok(RunnerWorkspace {
        _directory: directory,
        project_root,
        canonical_project,
    })
}

#[cfg(test)]
#[path = "test/runner_workspace.rs"]
mod tests;
