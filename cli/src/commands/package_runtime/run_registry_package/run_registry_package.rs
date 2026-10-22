//! Coordinates one native MGC package installation, verification and execution.
//! Điều phối một lượt MGC cài, xác minh và chạy package.

use anyhow::Result;
use std::path::{Path, PathBuf};

use super::super::work_root::PackageRunPurpose;

/// Resolved package identity after MGC install and execution.
/// Danh tính version package đã được MGC cài và chạy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegistryPackageReport {
    pub package: String,
    pub version: String,
}

/// Installs a registry package through MGC and runs its declared binary in the caller-owned directory.
/// Cài package registry qua MGC rồi chạy bin đã khai báo trong thư mục do caller quản lý.
pub(crate) async fn run_registry_package_in(
    package: String,
    selected_bin: Option<String>,
    allow_scripts: bool,
    args: Vec<String>,
    execution_cwd: &Path,
    purpose: PackageRunPurpose,
) -> Result<RegistryPackageReport> {
    let (pkg_name, version_tag) = super::super::request_validation::parse_request(&package)?;
    if selected_bin
        .as_deref()
        .is_some_and(|name| !super::super::bin_selection::valid_bin_name(name))
    {
        return Err(super::super::bin_selection::registry_package_invalid_bin());
    }
    let cwd = std::env::current_dir()?;
    let execution_cwd = canonical_execution_cwd(execution_cwd)?;
    let workspace = super::super::runner_workspace::prepare_runner_workspace(
        &cwd,
        pkg_name,
        version_tag,
        purpose,
    )?;
    super::super::native_install::install_mgc_package(
        &workspace,
        pkg_name,
        version_tag,
        purpose,
        allow_scripts,
    )?;
    let installed = super::super::installed_binary::verify_installed_binary(
        &workspace,
        pkg_name,
        selected_bin.as_deref(),
    )?;
    super::super::execute_package::execute_verified_binary(
        &installed,
        purpose,
        args,
        &execution_cwd,
    )?;
    Ok(RegistryPackageReport {
        package: pkg_name.to_string(),
        version: installed.package_version,
    })
}

fn canonical_execution_cwd(execution_cwd: &Path) -> Result<PathBuf> {
    let metadata = std::fs::symlink_metadata(execution_cwd)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(super::registry_package_execution_cwd_not_real());
    }
    execution_cwd.canonicalize().map_err(Into::into)
}

#[cfg(test)]
#[path = "test/run_registry_package.rs"]
mod tests;
