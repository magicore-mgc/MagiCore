//! Preflights source entries and confines link targets to the canonical source tree.
//! Kiểm tra trước các entry và giới hạn target link trong source tree canonical.

use std::fs;
use std::path::{Component, Path, PathBuf};

use super::error::SourceValidationError;

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

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
