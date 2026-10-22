//! Stable Flutter manifest facade over focused owner modules.
//! Facade manifest Flutter ổn định qua các module owner chuyên trách.

mod podfile;
mod pubspec;
mod sdk;

use mgc_types::{Manifest, MgResult};
use std::path::{Path, PathBuf};

pub fn parse_pubspec(project_root: &Path) -> MgResult<Manifest> {
    pubspec::parse_pubspec(project_root).map_err(mgc_types::MgError::from)
}

pub fn write_pubspec(project_root: &Path, manifest: &Manifest) -> MgResult<()> {
    pubspec::write_pubspec(project_root, manifest).map_err(mgc_types::MgError::from)
}

pub fn parse_podfile(project_root: &Path) -> MgResult<Manifest> {
    podfile::parse_podfile(project_root).map_err(mgc_types::MgError::from)
}

pub fn write_podfile(project_root: &Path, manifest: &Manifest) -> MgResult<()> {
    podfile::write_podfile(project_root, manifest).map_err(mgc_types::MgError::from)
}

pub(crate) fn expand_flutter_sdk_dependencies(
    project_root: &Path,
    manifest: &Manifest,
) -> MgResult<Manifest> {
    sdk::expand_flutter_sdk_dependencies(project_root, manifest).map_err(mgc_types::MgError::from)
}

pub(crate) fn flutter_sdk_packages(
    project_root: &Path,
) -> MgResult<(Option<PathBuf>, Vec<String>)> {
    sdk::flutter_sdk_packages(project_root).map_err(mgc_types::MgError::from)
}

pub(crate) fn has_flutter_sdk_dependencies(project_root: &Path) -> MgResult<bool> {
    sdk::has_flutter_sdk_dependencies(project_root).map_err(mgc_types::MgError::from)
}

pub(crate) fn flutter_sdk_package_names(
    project_root: &Path,
    sdk_root: &Path,
) -> MgResult<Vec<String>> {
    sdk::flutter_sdk_package_names(project_root, sdk_root).map_err(mgc_types::MgError::from)
}

#[cfg(test)]
pub(crate) fn expand_flutter_sdk_dependencies_with_root(
    project_root: &Path,
    manifest: &Manifest,
    sdk_root: &Path,
) -> MgResult<Manifest> {
    sdk::expand_flutter_sdk_dependencies_with_root(project_root, manifest, sdk_root)
        .map_err(mgc_types::MgError::from)
}

pub(crate) fn flutter_sdk_package_root(sdk_root: &Path, name: &str) -> MgResult<PathBuf> {
    sdk::flutter_sdk_package_root(sdk_root, name).map_err(mgc_types::MgError::from)
}

#[cfg(test)]
pub(crate) fn find_flutter_executable_in_path(
    path: &std::ffi::OsStr,
    pathext: Option<&std::ffi::OsStr>,
    windows: bool,
) -> Option<PathBuf> {
    sdk::find_flutter_executable_in_path(path, pathext, windows)
}
