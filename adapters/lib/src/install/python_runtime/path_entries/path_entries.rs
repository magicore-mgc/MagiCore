//! Coordinate Python manifest, lock ownership and wheel-path verification.
//! Điều phối kiểm tra manifest, lock owner và đường dẫn wheel Python.

use crate::install::shared_store;
use mgc_types::{MgError, MgResult};

use super::super::python_runtime::{python_project_owner, select_python_packages_for_owner};
use super::{
    lock_owner::read_python_lock,
    manifest_guard::{load_python_manifest, validate_manifest_pins},
    wheel_materialization::verify_python_package_materialization,
};

/// Importable site dirs for natively installed pure-python wheels, read
/// from the project's mgc.lock (no re-resolve). Runners (pytest/python)
/// prepend these to PYTHONPATH so `mgc test` sees mgc-owned packages —
/// the run side of "run tại MGC". A missing lock is a non-native project;
/// a malformed lock or missing materialization for a locked Python package
/// is an error, never a silently incomplete runtime.
/// (Site dir import được cho wheel pure-python đã cài native.)
pub fn native_python_path_entries(
    project_root: &std::path::Path,
) -> MgResult<Vec<std::path::PathBuf>> {
    let store = shared_store::SharedStoreRun::pypi()?;
    native_python_path_entries_from_store(project_root, &store.install_root)
}

/// Resolve and re-verify native Python runtime paths from an explicit MGC
/// PyPI store root. This explicit-root variant also makes store-integrity
/// tests hermetic instead of depending on the current user's home directory.
/// (Resolve và xác minh lại runtime Python từ gốc store PyPI tường minh.)
pub fn native_python_path_entries_from_store(
    project_root: &std::path::Path,
    pypi_store_root: &std::path::Path,
) -> MgResult<Vec<std::path::PathBuf>> {
    let python_manifest = load_python_manifest(project_root).map_err(MgError::from)?;
    let Some(lockfile) =
        read_python_lock(project_root, python_manifest.as_ref()).map_err(MgError::from)?
    else {
        return Ok(Vec::new());
    };
    let owner_core = python_project_owner(project_root)?;
    let python_packages = select_python_packages_for_owner(&lockfile.packages, &owner_core)?;
    validate_manifest_pins(python_manifest.as_ref(), &python_packages, &owner_core)
        .map_err(MgError::from)?;
    if python_packages.is_empty() {
        return Ok(Vec::new());
    }

    let wheels_root = pypi_store_root.join("wheels");
    let site = wheels_root.join("site");
    python_packages
        .into_iter()
        .map(|package| {
            verify_python_package_materialization(package, &wheels_root, &site, &owner_core)
                .map_err(MgError::from)
        })
        .collect()
}
