//! Verifies installed package identity and that the selected executable stays owned.
//! Xác minh identity package và executable được chọn vẫn thuộc vùng cài đặt.

use anyhow::Result;
use std::path::PathBuf;

use super::super::runner_workspace::RunnerWorkspace;

pub(in crate::commands::package_runtime) struct VerifiedBinary {
    pub(in crate::commands::package_runtime) package_version: String,
    #[cfg(test)]
    pub(in crate::commands::package_runtime) package_root: PathBuf,
    pub(in crate::commands::package_runtime) bin_dir: PathBuf,
    #[cfg(test)]
    pub(in crate::commands::package_runtime) bin_name: String,
    pub(in crate::commands::package_runtime) bin_path: PathBuf,
    pub(in crate::commands::package_runtime) canonical_target: PathBuf,
}

pub(in crate::commands::package_runtime) fn verify_installed_binary(
    workspace: &RunnerWorkspace,
    package: &str,
    selected_bin: Option<&str>,
) -> Result<VerifiedBinary> {
    let bin_dir = workspace.project_root.join("node_modules").join(".bin");
    let package_root = workspace
        .project_root
        .join("node_modules")
        .join(package)
        .canonicalize()?;
    if !package_root.starts_with(&workspace.canonical_project) {
        return Err(super::registry_package_binary_outside_project());
    }
    let package_version =
        super::super::installed_metadata::requested_package_version(&package_root, package)?;
    let entries = super::super::installed_metadata::requested_package_bins(&package_root, package)?;
    let bin_name = super::super::bin_selection::select_bin_name(&entries, package, selected_bin)?;
    let declared_target = entries
        .iter()
        .find(|(name, _)| name == &bin_name)
        .ok_or_else(super::super::bin_selection::registry_package_undeclared_bin)?
        .1
        .clone();
    let canonical_target = package_root.join(declared_target).canonicalize()?;
    if !canonical_target.starts_with(&package_root) || !canonical_target.is_file() {
        return Err(super::registry_package_bin_target_mismatch());
    }
    let bin_path = bin_dir.join(&bin_name);
    #[cfg(windows)]
    let bin_path = {
        let windows_shim = bin_dir.join(format!("{bin_name}.cmd"));
        if windows_shim.is_file() {
            windows_shim
        } else {
            bin_path
        }
    };
    if !bin_path.is_file() {
        return Err(super::registry_package_no_binary(&bin_name, package));
    }
    if !bin_path
        .canonicalize()?
        .starts_with(&workspace.canonical_project)
    {
        return Err(super::registry_package_binary_outside_project());
    }
    #[cfg(unix)]
    if bin_path.canonicalize()? != canonical_target {
        return Err(super::registry_package_bin_target_mismatch());
    }
    Ok(VerifiedBinary {
        package_version,
        #[cfg(test)]
        package_root,
        bin_dir,
        #[cfg(test)]
        bin_name,
        bin_path,
        canonical_target,
    })
}

#[cfg(test)]
#[path = "test/installed_binary.rs"]
mod tests;
