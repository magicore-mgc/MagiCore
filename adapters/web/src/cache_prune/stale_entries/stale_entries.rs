//! Remove stale files, unpinned package roots, and metadata entries.
//! Xóa file cũ, package root không được ghim và metadata hết hạn.

use std::path::{Path, PathBuf};

use mgc_types::{MgError, MgResult};
use walkdir::WalkDir;

use crate::cache_prune::{canonical_or_original, path_is_older_than, remove_dir_if_empty};

pub fn prune_old_files_under(root: &Path, max_age: std::time::Duration) -> MgResult<()> {
    if !root.exists() {
        return Ok(());
    }
    let mut directories = Vec::new();
    for entry in WalkDir::new(root)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.file_type().is_dir() {
            directories.push(entry.path().to_path_buf());
            continue;
        }
        if !entry.file_type().is_file() {
            continue;
        }
        if path_is_older_than(entry.path(), max_age) {
            let _ = std::fs::remove_file(entry.path());
        }
    }
    directories.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
    for dir in directories {
        remove_dir_if_empty(&dir);
    }
    Ok(())
}

pub fn prune_old_package_dirs_under(
    root: &Path,
    max_age: std::time::Duration,
    pinned_package_roots: &std::collections::HashSet<PathBuf>,
) -> MgResult<()> {
    if !root.exists() {
        return Ok(());
    }
    let mut directories = Vec::new();
    for entry in WalkDir::new(root)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.file_type().is_dir() {
            directories.push(entry.path().to_path_buf());
        }
    }
    directories.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
    for dir in directories {
        if pinned_package_roots.contains(&canonical_or_original(&dir)) {
            continue;
        }
        let marker = dir.join(".magicore-package-root.json");
        let package_json = dir.join("package.json");
        if marker.exists() || package_json.exists() {
            if path_is_older_than(
                if marker.exists() {
                    &marker
                } else {
                    &package_json
                },
                max_age,
            ) {
                let _ = std::fs::remove_dir_all(&dir);
            }
        } else {
            remove_dir_if_empty(&dir);
        }
    }
    Ok(())
}

pub fn prune_old_metadata_dirs_under(root: &Path, max_age: std::time::Duration) -> MgResult<()> {
    if !root.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(root).map_err(|err| {
        MgError::Other(format!(
            "failed to read metadata cache root '{}': {}",
            root.display(),
            err
        ))
    })? {
        let Ok(entry) = entry else {
            continue;
        };
        let dir = entry.path();
        let metadata_json = dir.join("metadata.json");
        if metadata_json.exists() && path_is_older_than(&metadata_json, max_age) {
            let _ = std::fs::remove_dir_all(&dir);
        }
    }
    Ok(())
}
