//! Resolve requested package subpaths inside a canonical package root.
//! Resolve subpath package được request bên trong canonical package root.

use std::path::{Path, PathBuf};

use super::super::{PackageEntryError, canonicalize_if_present};

pub(in crate::bundler::dependency_cache::package_entry::implementation) enum PackageSubpath {
    File(PathBuf),
    Directory(PathBuf),
}

pub(in crate::bundler::dependency_cache::package_entry::implementation) fn split_package_name_and_subpath(
    package_identifier: &str,
) -> Result<(PathBuf, PathBuf), PackageEntryError> {
    let mut segments = package_identifier.split('/');
    let Some(first) = segments.next() else {
        return Err(PackageEntryError::InvalidPackageName);
    };

    let mut package_name = PathBuf::from(first);
    if first.starts_with('@') {
        let Some(name) = segments.next() else {
            return Err(PackageEntryError::InvalidPackageName);
        };
        package_name.push(name);
    }

    let mut subpath = PathBuf::new();
    for segment in segments {
        subpath.push(segment);
    }
    Ok((package_name, subpath))
}

pub(in crate::bundler::dependency_cache::package_entry::implementation) fn resolve_package_subpath(
    package_root: &Path,
    package_subpath: &Path,
) -> Result<Option<PackageSubpath>, PackageEntryError> {
    let Some(path) = canonicalize_if_present(&package_root.join(package_subpath))? else {
        return Ok(None);
    };
    if !path.starts_with(package_root) {
        return Err(PackageEntryError::ForbiddenPath);
    }

    let metadata = std::fs::metadata(&path).map_err(PackageEntryError::Io)?;
    if metadata.is_file() {
        Ok(Some(PackageSubpath::File(path)))
    } else if metadata.is_dir() {
        Ok(Some(PackageSubpath::Directory(path)))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
