//! Resolve regular files without allowing paths to leave a package root.
//! Resolve regular file nhưng không cho path thoát package root.

use std::{
    fs,
    io::ErrorKind,
    path::{Component, Path, PathBuf},
};

use super::super::PackageEntryError;

pub(in crate::bundler::dependency_cache::package_entry::implementation) fn resolve_file_inside_package(
    package_root: &Path,
    candidate: &Path,
) -> Result<Option<PathBuf>, PackageEntryError> {
    if !is_lexically_inside_package(package_root, candidate) {
        return Err(PackageEntryError::ForbiddenPath);
    }

    let Some(canonical_file) = canonicalize_if_present(candidate)? else {
        return Ok(None);
    };
    if !canonical_file.starts_with(package_root) {
        return Err(PackageEntryError::ForbiddenPath);
    }

    let metadata = fs::metadata(&canonical_file).map_err(PackageEntryError::Io)?;
    if metadata.is_file() {
        Ok(Some(canonical_file))
    } else {
        Ok(None)
    }
}

/// Report whether a canonical directory exists inside the package root.
/// Kiểm tra thư mục canonical có tồn tại bên trong package root hay không.
///
/// A missing directory is a normal miss, but a directory cannot serve as a
/// module entry: callers must fall through to file candidates.
/// (Thiếu thư mục là miss bình thường, nhưng thư mục không thể làm entry
/// module: caller phải thử tiếp các candidate file.)
pub(in crate::bundler::dependency_cache::package_entry::implementation) fn directory_inside_package(
    package_root: &Path,
    candidate: &Path,
) -> Result<bool, PackageEntryError> {
    if !is_lexically_inside_package(package_root, candidate) {
        return Err(PackageEntryError::ForbiddenPath);
    }

    let Some(canonical_dir) = canonicalize_if_present(candidate)? else {
        return Ok(false);
    };
    if !canonical_dir.starts_with(package_root) {
        return Err(PackageEntryError::ForbiddenPath);
    }
    Ok(fs::metadata(&canonical_dir)
        .map_err(PackageEntryError::Io)?
        .is_dir())
}

fn is_lexically_inside_package(package_root: &Path, candidate: &Path) -> bool {
    let Ok(relative) = candidate.strip_prefix(package_root) else {
        return false;
    };
    let mut depth = 0usize;
    for component in relative.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(_) => depth += 1,
            Component::ParentDir if depth > 0 => depth -= 1,
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return false,
        }
    }
    true
}

pub(in crate::bundler::dependency_cache::package_entry::implementation) fn canonicalize_if_present(
    path: &Path,
) -> Result<Option<PathBuf>, PackageEntryError> {
    // A NUL byte can only come from a lossy conversion of a non-UTF-8
    // request segment; the OS rejects it as InvalidInput, not NotFound.
    // (Byte NUL chỉ đến từ chuyển lossy của segment request không phải
    // UTF-8; OS báo InvalidInput chứ không phải NotFound nên phải là miss.)
    if path.to_string_lossy().contains('\0') {
        return Ok(None);
    }
    match fs::canonicalize(path) {
        Ok(path) => Ok(Some(path)),
        Err(error) if matches!(error.kind(), ErrorKind::NotFound | ErrorKind::NotADirectory) => {
            Ok(None)
        }
        Err(error) => Err(PackageEntryError::Io(error)),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
