//! Project a canonical destination path without creating it.
//! Tính path destination canonical mà không tạo thư mục.

use std::fs;
use std::path::{Path, PathBuf};

use super::super::error::DestinationError;

pub(in crate::commands::store) fn prospective_destination_root(
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
    let mut cursor = absolute.clone();
    let mut missing = Vec::new();
    loop {
        match fs::symlink_metadata(&cursor) {
            Ok(metadata) if missing.is_empty() && metadata.file_type().is_symlink() => {
                let parent = cursor
                    .parent()
                    .ok_or_else(|| DestinationError::NotDirectory(cursor.clone()))?;
                let name = cursor
                    .file_name()
                    .ok_or_else(|| DestinationError::NotDirectory(cursor.clone()))?;
                let canonical_parent = fs::canonicalize(parent).map_err(|source| {
                    DestinationError::CanonicalizeDirectory {
                        path: parent.to_path_buf(),
                        source,
                    }
                })?;
                return Ok(canonical_parent.join(name));
            }
            Ok(metadata) if missing.is_empty() && !metadata.is_dir() => {
                return Err(DestinationError::NotDirectory(cursor));
            }
            Ok(_) => {
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
                let mut result = canonical;
                for component in missing.iter().rev() {
                    result.push(component);
                }
                return Ok(result);
            }
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
    }
}
