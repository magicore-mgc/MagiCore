//! Validates destination entry types and measures regular backup files.
//! Xác thực loại entry đích và đo kích thước file backup thường.

use std::fs;
use std::path::{Path, PathBuf};

use super::super::super::backup_storage::{
    ensure_empty_destination_directory, prepare_destination_root, prospective_destination_root,
};
use super::super::super::paths_overlap;
use super::super::error::BackupError;

pub(in crate::commands::store::backup) fn validate_backup_destination(
    requested_backup_dir: &Path,
    cache_root: Option<&Path>,
    vstore_source: Option<&Path>,
) -> Result<PathBuf, BackupError> {
    let vstore_destination = requested_backup_dir.join("vstore");
    let database_destination = requested_backup_dir.join("store.db");
    validate_destination_entry(requested_backup_dir, true)?;
    validate_destination_entry(&vstore_destination, true)?;
    validate_destination_entry(&database_destination, false)?;
    let backup_candidate =
        prospective_destination_root(requested_backup_dir).map_err(|source| {
            BackupError::TreeOperation {
                operation: "resolve the requested backup destination",
                source: anyhow::Error::new(source),
            }
        })?;
    if cache_root.is_some_and(|root| paths_overlap(&backup_candidate, root))
        || vstore_source.is_some_and(|root| paths_overlap(&backup_candidate, root))
    {
        return Err(BackupError::BackupDestinationOverlap {
            destination: backup_candidate,
        });
    }
    Ok(backup_candidate)
}

pub(in crate::commands::store::backup) fn prepare_backup_destination(
    requested_backup_dir: &Path,
    uses_default_backup_path: bool,
    backup_candidate: &Path,
    cache_root: Option<&Path>,
    vstore_source: Option<&Path>,
) -> Result<PathBuf, BackupError> {
    if uses_default_backup_path {
        let default_root =
            requested_backup_dir
                .parent()
                .ok_or_else(|| BackupError::TreeOperation {
                    operation: "resolve the default backup parent",
                    source: anyhow::anyhow!("default backup path has no parent"),
                })?;
        prepare_destination_root(default_root).map_err(|source| BackupError::TreeOperation {
            operation: "prepare the default backup parent without following its final component",
            source: anyhow::Error::new(source),
        })?;
    }

    let backup_root = prepare_destination_root(requested_backup_dir).map_err(|source| {
        BackupError::TreeOperation {
            operation: "prepare the backup directory",
            source: anyhow::Error::new(source),
        }
    })?;
    if backup_root.as_path() != backup_candidate {
        return Err(BackupError::BackupDestinationChanged {
            expected: backup_candidate.to_path_buf(),
            actual: backup_root,
        });
    }
    if cache_root.is_some_and(|root| paths_overlap(&backup_root, root))
        || vstore_source.is_some_and(|root| paths_overlap(&backup_root, root))
    {
        return Err(BackupError::BackupDestinationOverlap {
            destination: backup_root,
        });
    }
    ensure_empty_destination_directory(&backup_root).map_err(|source| {
        BackupError::TreeOperation {
            operation: "check that the backup destination is empty",
            source: anyhow::Error::new(source),
        }
    })?;
    Ok(backup_root)
}

fn validate_destination_entry(path: &Path, expects_directory: bool) -> Result<(), BackupError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            Err(BackupError::InvalidBackupEntry(path.to_path_buf()))
        }
        Ok(metadata) if expects_directory && !metadata.is_dir() => {
            Err(BackupError::InvalidBackupEntry(path.to_path_buf()))
        }
        Ok(metadata) if !expects_directory && !metadata.is_file() => {
            Err(BackupError::InvalidBackupEntry(path.to_path_buf()))
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(BackupError::InspectBackupEntry {
            path: path.to_path_buf(),
            source,
        }),
    }
}

pub(in crate::commands::store::backup) fn regular_file_len(
    path: &Path,
) -> Result<u64, BackupError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            Err(BackupError::InvalidBackupEntry(path.to_path_buf()))
        }
        Ok(metadata) => Ok(metadata.len()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(0),
        Err(source) => Err(BackupError::InspectBackupEntry {
            path: path.to_path_buf(),
            source,
        }),
    }
}
