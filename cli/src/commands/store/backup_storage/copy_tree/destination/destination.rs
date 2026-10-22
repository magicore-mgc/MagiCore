//! Prepare a real destination root and confirm its final filesystem identity.
//! Chuẩn bị root đích thật và xác nhận danh tính filesystem cuối.

use std::fs;
use std::path::{Path, PathBuf};

use super::error::DestinationError;

pub(in crate::commands::store) fn prepare_destination_root(
    destination: &Path,
) -> Result<PathBuf, DestinationError> {
    let absolute = if destination.is_absolute() {
        destination.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|source| DestinationError::CanonicalizeDirectory {
                path: destination.to_path_buf(),
                source,
            })?
            .join(destination)
    };
    match fs::symlink_metadata(&absolute) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(DestinationError::SymlinkRoot(absolute));
        }
        Ok(metadata) if !metadata.is_dir() => {
            return Err(DestinationError::NotDirectory(absolute));
        }
        Ok(_) => return canonical_real_destination(&absolute),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(source) => {
            return Err(DestinationError::InspectRoot {
                path: absolute,
                source,
            });
        }
    }

    // Resolve existing ancestors once, then create the missing suffix beneath that canonical
    // location. This keeps valid platform aliases such as macOS /tmp while refusing a symlink
    // at the destination root itself and every newly-created child.
    let mut cursor = absolute.clone();
    let mut missing = Vec::new();
    let mut canonical_ancestor = loop {
        match fs::symlink_metadata(&cursor) {
            Ok(metadata) if metadata.is_dir() || metadata.file_type().is_symlink() => {
                let canonical = fs::canonicalize(&cursor).map_err(|source| {
                    DestinationError::CanonicalizeDirectory {
                        path: cursor.clone(),
                        source,
                    }
                })?;
                if !fs::metadata(&canonical)
                    .map_err(|source| DestinationError::InspectDirectory {
                        path: canonical.clone(),
                        source,
                    })?
                    .is_dir()
                {
                    return Err(DestinationError::NotDirectory(cursor));
                }
                break canonical;
            }
            Ok(_) => return Err(DestinationError::NotDirectory(cursor)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let name = cursor
                    .file_name()
                    .ok_or_else(|| DestinationError::NotDirectory(cursor.clone()))?;
                missing.push(name.to_os_string());
                cursor = cursor
                    .parent()
                    .ok_or_else(|| DestinationError::NotDirectory(absolute.clone()))?
                    .to_path_buf();
            }
            Err(source) => {
                return Err(DestinationError::InspectDirectory {
                    path: cursor,
                    source,
                });
            }
        }
    };

    for name in missing.iter().rev() {
        let child = canonical_ancestor.join(name);
        match fs::create_dir(&child) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(source) => {
                return Err(DestinationError::CreateDirectory {
                    path: child,
                    source,
                });
            }
        }
        let metadata =
            fs::symlink_metadata(&child).map_err(|source| DestinationError::InspectDirectory {
                path: child.clone(),
                source,
            })?;
        if metadata.file_type().is_symlink() {
            return Err(DestinationError::SymlinkDirectory(child));
        }
        if !metadata.is_dir() {
            return Err(DestinationError::NotRealDirectory(child));
        }
        canonical_ancestor =
            fs::canonicalize(&child).map_err(|source| DestinationError::CanonicalizeDirectory {
                path: child,
                source,
            })?;
    }
    canonical_real_destination(&canonical_ancestor)
}

fn canonical_real_destination(path: &Path) -> Result<PathBuf, DestinationError> {
    let canonical =
        fs::canonicalize(path).map_err(|source| DestinationError::CanonicalizeRoot {
            path: path.to_path_buf(),
            source,
        })?;
    let confirmed = fs::symlink_metadata(path).map_err(|source| DestinationError::InspectRoot {
        path: path.to_path_buf(),
        source,
    })?;
    if confirmed.file_type().is_symlink() {
        return Err(DestinationError::SymlinkRoot(path.to_path_buf()));
    }
    if !confirmed.is_dir() {
        return Err(DestinationError::NotDirectory(path.to_path_buf()));
    }
    Ok(canonical)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
