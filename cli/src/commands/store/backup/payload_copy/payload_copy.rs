//! Copies validated database/tree payload and measures the completed backup.
//! Copy database/cây đã xác thực và đo payload backup hoàn tất.

use std::fs;
use std::path::Path;

use crate::commands::safe_filesystem::{copy_regular_file_no_follow, logical_payload_bytes};
use crate::commands::store::backup_storage::{ValidatedSourceTree, copy_validated_tree};

use super::super::destination_validation::regular_file_len;
use super::super::error::BackupError;

pub(in crate::commands::store::backup) struct BackupPayload {
    pub(in crate::commands::store::backup) database_bytes: u64,
    pub(in crate::commands::store::backup) vstore_bytes: u64,
}

pub(in crate::commands::store::backup) fn copy_backup_payload(
    backup_root: &Path,
    database_source: Option<&Path>,
    validated_vstore: Option<&ValidatedSourceTree>,
) -> Result<BackupPayload, BackupError> {
    if let Some(source) = validated_vstore {
        copy_validated_tree(source, &backup_root.join("vstore")).map_err(|source| {
            BackupError::TreeOperation {
                operation: "copy the project virtual store",
                source,
            }
        })?;
    }

    let database_destination = backup_root.join("store.db");
    if let Some(source) = database_source {
        copy_regular_file_no_follow(source, &database_destination).map_err(|source_error| {
            BackupError::DatabaseCopy {
                source_path: source.to_path_buf(),
                destination_path: database_destination.clone(),
                source: source_error,
            }
        })?;
    }

    let database_bytes = regular_file_len(&database_destination)?;
    let vstore_destination = backup_root.join("vstore");
    let vstore_bytes = match fs::symlink_metadata(&vstore_destination) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            return Err(BackupError::InvalidBackupEntry(vstore_destination));
        }
        Ok(_) => logical_payload_bytes(&vstore_destination).map_err(|source| {
            BackupError::MeasurePayload {
                path: vstore_destination.clone(),
                source: anyhow::Error::new(source),
            }
        })?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => 0,
        Err(source) => {
            return Err(BackupError::InspectBackupEntry {
                path: vstore_destination,
                source,
            });
        }
    };

    Ok(BackupPayload {
        database_bytes,
        vstore_bytes,
    })
}
