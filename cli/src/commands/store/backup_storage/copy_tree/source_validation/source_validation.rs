//! Preflights source entries and confines link targets to the canonical source tree.
//! Kiểm tra trước các entry và giới hạn target link trong source tree canonical.
use std::fs;
use std::path::{Path, PathBuf};

use super::error::SourceValidationError;

#[path = "path_check/mod.rs"]
mod path_check;

pub(in crate::commands::store) use path_check::{validate_real_directory, validated_link_target};

pub(in crate::commands::store) struct ValidatedSourceTree {
    root: PathBuf,
}

impl ValidatedSourceTree {
    pub(in crate::commands::store::backup_storage::copy_tree) fn root(&self) -> &Path {
        &self.root
    }
}

pub(in crate::commands::store) fn validate_present_source_tree(
    source: &Path,
) -> Result<ValidatedSourceTree, SourceValidationError> {
    let source_root = inspect_source_root(source)?
        .ok_or_else(|| SourceValidationError::MissingRoot(source.to_path_buf()))?;
    validate_tree_root(&source_root)?;
    Ok(ValidatedSourceTree { root: source_root })
}

pub(super) fn validate_tree_root(source_root: &Path) -> Result<(), SourceValidationError> {
    validate_source_directory(source_root, source_root)
}

fn validate_source_directory(
    source_root: &Path,
    directory: &Path,
) -> Result<(), SourceValidationError> {
    validate_real_directory(directory, source_root)?;
    for entry in fs::read_dir(directory).map_err(|source| SourceValidationError::ReadDirectory {
        path: directory.to_path_buf(),
        source,
    })? {
        let entry = entry.map_err(|source| SourceValidationError::ReadEntry {
            path: directory.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        let file_type =
            entry
                .file_type()
                .map_err(|source| SourceValidationError::InspectEntry {
                    path: path.clone(),
                    source,
                })?;
        if file_type.is_symlink() {
            let _ = validated_link_target(source_root, &path)?;
        } else if file_type.is_dir() {
            validate_source_directory(source_root, &path)?;
        } else if file_type.is_file() {
            let _ = mgc_platform::open_regular_file_no_follow(&path).map_err(|source| {
                SourceValidationError::OpenSource {
                    path: path.clone(),
                    source,
                }
            })?;
        } else {
            return Err(SourceValidationError::SpecialEntry(path));
        }
    }
    Ok(())
}

pub(super) fn inspect_source_root(source: &Path) -> Result<Option<PathBuf>, SourceValidationError> {
    let metadata = match fs::symlink_metadata(source) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source_error) => {
            return Err(SourceValidationError::InspectRoot {
                path: source.to_path_buf(),
                source: source_error,
            });
        }
    };
    if metadata.file_type().is_symlink() {
        return Err(SourceValidationError::SymlinkRoot(source.to_path_buf()));
    }
    if !metadata.is_dir() {
        return Err(SourceValidationError::NotDirectory(source.to_path_buf()));
    }
    let canonical = fs::canonicalize(source).map_err(|source_error| {
        SourceValidationError::CanonicalizeRoot {
            path: source.to_path_buf(),
            source: source_error,
        }
    })?;
    let confirmed = fs::symlink_metadata(source).map_err(|source_error| {
        SourceValidationError::InspectRoot {
            path: source.to_path_buf(),
            source: source_error,
        }
    })?;
    if confirmed.file_type().is_symlink() {
        return Err(SourceValidationError::SymlinkRoot(source.to_path_buf()));
    }
    if !confirmed.is_dir() {
        return Err(SourceValidationError::NotDirectory(source.to_path_buf()));
    }
    Ok(Some(canonical))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
