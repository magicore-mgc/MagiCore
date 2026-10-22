//! Virtual store operations for Web dependency materialization.
//! Thao tác virtual store thuộc materialize dependency Web.

use std::path::{Path, PathBuf};

use mgc_types::PackageId;

pub fn strict_vstore_package_dir(node_modules: &Path, package_id: &PackageId) -> PathBuf {
    strict_vstore_node_modules_dir(node_modules, package_id).join(package_id.name().as_str())
}

pub fn strict_vstore_node_modules_dir(node_modules: &Path, package_id: &PackageId) -> PathBuf {
    let vstore_pkg_name = format!(
        "{}@{}",
        package_id.name().as_str().replace('/', "+"),
        package_id.version()
    );
    node_modules
        .join(".magicore")
        .join(vstore_pkg_name)
        .join("node_modules")
}

#[cfg(test)]
#[path = "test/virtual_store.rs"]
mod tests;
