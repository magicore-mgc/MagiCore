//! Wires Python installed-package listing.
//! Wiring liệt kê package Python đã cài.

mod error;
mod installed_packages;

use mgc_types::{MgError, MgResult, adapter::InstalledPackage};
use std::path::Path;

pub(crate) fn native_python_installed_packages(
    project_root: &Path,
    manifest: &mgc_types::Manifest,
) -> MgResult<Vec<InstalledPackage>> {
    installed_packages::list_native_python_installed_packages(project_root, manifest)
        .map_err(MgError::from)
}
