//! Prepare an existing or missing destination parent.
//! Chuẩn bị parent destination đã có hoặc còn thiếu.

use std::fs;
use std::path::{Path, PathBuf};

use super::super::destination::prepare_destination_root;
use super::super::error::DestinationError;

pub(in crate::commands::store::backup_storage::copy_tree) fn prepare_destination_parent(
    parent: &Path,
) -> Result<PathBuf, DestinationError> {
    match fs::symlink_metadata(parent) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            let canonical = fs::canonicalize(parent).map_err(|source| {
                DestinationError::CanonicalizeDirectory {
                    path: parent.to_path_buf(),
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
                return Err(DestinationError::NotDirectory(parent.to_path_buf()));
            }
            Ok(canonical)
        }
        Ok(_) => prepare_destination_root(parent),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            prepare_destination_root(parent)
        }
        Err(source) => Err(DestinationError::InspectRoot {
            path: parent.to_path_buf(),
            source,
        }),
    }
}
