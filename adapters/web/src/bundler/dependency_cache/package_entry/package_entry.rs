//! Resolve a requested package or subpath to a canonical installed file.
//! Resolve package/subpath được request thành file canonical đã cài.

use std::{
    fs,
    path::{Path, PathBuf},
};

use self::manifest_entry::resolve_manifest_entry;
use self::package_selector::{
    PackageSubpath, resolve_package_subpath, split_package_name_and_subpath,
};
use self::path_containment::{canonicalize_if_present, resolve_file_inside_package};
use super::super::name_validation::dep_name_is_valid;
use super::PackageEntryError;

#[path = "manifest_entry/mod.rs"]
mod manifest_entry;
#[path = "package_selector/mod.rs"]
mod package_selector;
#[path = "path_containment/mod.rs"]
mod path_containment;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

/// Find a package/subpath file using browser/module/main/index precedence.
/// Tìm file package/subpath theo thứ tự browser/module/main/index.
///
/// A top-level package directory may be a pnpm/workspace symlink. Its
/// resolved directory becomes the containment root; a requested subpath
/// and its selected entry must remain under that root.
/// Package top-level có thể là symlink pnpm/workspace. Thư mục sau resolve
/// là root containment; subpath request và entry phải còn trong root đó.
pub(in crate::bundler::dependency_cache) fn resolve_package_entry(
    node_modules: &Path,
    pkg_name: &str,
) -> Result<Option<PathBuf>, PackageEntryError> {
    if !dep_name_is_valid(pkg_name) {
        return Err(PackageEntryError::InvalidPackageName);
    }

    let (package_name, package_subpath) = split_package_name_and_subpath(pkg_name)?;
    let package_root = match canonicalize_if_present(&node_modules.join(package_name))? {
        Some(path) => path,
        None => return Ok(None),
    };
    if !fs::metadata(&package_root)
        .map_err(PackageEntryError::Io)?
        .is_dir()
    {
        return Ok(None);
    }

    let selected = if package_subpath.as_os_str().is_empty() {
        PackageSubpath::Directory(package_root.clone())
    } else {
        let Some(selected) = resolve_package_subpath(&package_root, &package_subpath)? else {
            return Ok(None);
        };
        selected
    };
    let selected_directory = match selected {
        PackageSubpath::File(path) => return Ok(Some(path)),
        PackageSubpath::Directory(path) => path,
    };

    resolve_manifest_entry(&package_root, &selected_directory)
}
