//! List only locked, verified Python packages owned by this core.
//! Liệt kê package Python đã lock, xác minh và thuộc core này.

use crate::install::{native_python_path_entries, read_existing_lock};
use std::path::Path;

use super::super::python_runtime::{python_project_owner, select_python_packages_for_owner};
use super::error::PythonInstalledPackagesError;

/// MGC-owned Python installed set. Unlike a toolchain venv, native wheels
/// live in the shared managed site tree; report only exact Python pins in
/// `mgc.lock` whose importable materialization was verified above.
/// (Danh sách package Python MGC sở hữu: chỉ pin mgc.lock có site đã kiểm.)
pub(super) fn list_native_python_installed_packages(
    project_root: &Path,
    manifest: &mgc_types::Manifest,
) -> Result<Vec<mgc_types::adapter::InstalledPackage>, PythonInstalledPackagesError> {
    // Listing reports only packages MGC has actually locked/materialized. A
    // Python manifest without mgc.lock is not a runtime launch request, so it
    // must not turn the honest empty installed set into an error.
    // (List chỉ báo package do MGC quản lý; project chưa có lock thì tập đã
    // cài của MGC là rỗng, không phải lỗi runtime.)
    match std::fs::symlink_metadata(project_root.join("mgc.lock")) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(source) => return Err(PythonInstalledPackagesError::LockfileInspection { source }),
        Ok(_) => {}
    }
    let lockfile = read_existing_lock(project_root)?;
    let owner_core = python_project_owner(project_root)?;
    let python_packages = select_python_packages_for_owner(&lockfile.packages, &owner_core)?;
    let paths = native_python_path_entries(project_root)?;
    if python_packages.len() != paths.len() {
        return Err(PythonInstalledPackagesError::LockMaterializationChanged);
    }
    python_packages
        .into_iter()
        .zip(paths)
        .map(|(package, path)| {
            let name = mgc_types::PackageName::new(&package.name)?;
            let version = mgc_types::Version::parse(&package.version)?;
            let is_direct = manifest
                .all_dependencies()
                .any(|dependency| dependency.name == name);
            Ok(mgc_types::adapter::InstalledPackage {
                id: mgc_types::PackageId::new(name, version),
                path,
                integrity: (!package.integrity.is_empty()).then(|| package.integrity.clone()),
                is_direct,
                is_dev: false,
            })
        })
        .collect()
}
