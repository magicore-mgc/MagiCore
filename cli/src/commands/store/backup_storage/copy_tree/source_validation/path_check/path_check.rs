//! Real-directory and link-target safety checks for backup copy sources.
//! Kiểm an toàn thư mục thật và đích link cho nguồn copy backup.

use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::commands::store::backup_storage::copy_tree::source_validation::error::SourceValidationError;

pub(in crate::commands::store) fn validate_real_directory(
    path: &Path,
    root: &Path,
) -> Result<(), SourceValidationError> {
    let metadata =
        fs::symlink_metadata(path).map_err(|source| SourceValidationError::InspectDirectory {
            path: path.to_path_buf(),
            source,
        })?;
    if metadata.file_type().is_symlink() {
        return Err(SourceValidationError::SymlinkDirectory(path.to_path_buf()));
    }
    if !metadata.is_dir() {
        return Err(SourceValidationError::NotRealDirectory(path.to_path_buf()));
    }
    let canonical =
        fs::canonicalize(path).map_err(|source| SourceValidationError::CanonicalizeDirectory {
            path: path.to_path_buf(),
            source,
        })?;
    if !canonical.starts_with(root) {
        return Err(SourceValidationError::DirectoryEscapesRoot {
            path: canonical,
            root: root.to_path_buf(),
        });
    }
    let confirmed =
        fs::symlink_metadata(path).map_err(|source| SourceValidationError::InspectDirectory {
            path: path.to_path_buf(),
            source,
        })?;
    if confirmed.file_type().is_symlink() {
        return Err(SourceValidationError::SymlinkDirectory(path.to_path_buf()));
    }
    if !confirmed.is_dir() {
        return Err(SourceValidationError::NotRealDirectory(path.to_path_buf()));
    }
    Ok(())
}

pub(in crate::commands::store) fn validated_link_target(
    source_root: &Path,
    source_link: &Path,
) -> Result<(PathBuf, bool), SourceValidationError> {
    let raw_target =
        fs::read_link(source_link).map_err(|source| SourceValidationError::ReadLink {
            path: source_link.to_path_buf(),
            source,
        })?;
    let target_path = if raw_target.is_absolute() {
        raw_target.clone()
    } else {
        let relative_target = source_link
            .parent()
            .unwrap_or(source_root)
            .strip_prefix(source_root)
            .map_err(|_| SourceValidationError::LinkEscapesRoot {
                link: source_link.to_path_buf(),
                target: source_link.to_path_buf(),
                root: source_root.to_path_buf(),
            })?
            .join(raw_target);
        let mut suffix = PathBuf::new();
        for component in relative_target.components() {
            match component {
                Component::CurDir => {}
                Component::Normal(part) => suffix.push(part),
                Component::ParentDir if suffix.pop() => {}
                Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                    return Err(SourceValidationError::LinkEscapesRoot {
                        link: source_link.to_path_buf(),
                        target: source_root.join(relative_target),
                        root: source_root.to_path_buf(),
                    });
                }
            }
        }
        source_root.join(suffix)
    };
    let canonical_target =
        fs::canonicalize(&target_path).map_err(|source| SourceValidationError::ResolveLink {
            path: source_link.to_path_buf(),
            source,
        })?;
    if !canonical_target.starts_with(source_root) {
        return Err(SourceValidationError::LinkEscapesRoot {
            link: source_link.to_path_buf(),
            target: canonical_target,
            root: source_root.to_path_buf(),
        });
    }
    let target_metadata =
        fs::metadata(&canonical_target).map_err(|source| SourceValidationError::ResolveLink {
            path: source_link.to_path_buf(),
            source,
        })?;
    if !target_metadata.is_file() && !target_metadata.is_dir() {
        return Err(SourceValidationError::UnsupportedLinkTarget {
            link: source_link.to_path_buf(),
            target: canonical_target,
        });
    }
    let target_suffix = canonical_target
        .strip_prefix(source_root)
        .map_err(|_| SourceValidationError::LinkEscapesRoot {
            link: source_link.to_path_buf(),
            target: canonical_target.clone(),
            root: source_root.to_path_buf(),
        })?
        .to_path_buf();
    if target_metadata.is_dir() {
        let link_parent = source_link.parent().unwrap_or(source_root);
        let link_parent_suffix = link_parent.strip_prefix(source_root).map_err(|_| {
            SourceValidationError::LinkEscapesRoot {
                link: source_link.to_path_buf(),
                target: link_parent.to_path_buf(),
                root: source_root.to_path_buf(),
            }
        })?;
        if link_parent_suffix.starts_with(&target_suffix) {
            return Err(SourceValidationError::CyclicDirectoryLink {
                link: source_link.to_path_buf(),
                target: canonical_target,
            });
        }
    }
    Ok((target_suffix, target_metadata.is_dir()))
}
