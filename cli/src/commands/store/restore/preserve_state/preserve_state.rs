//! Copies only existing project state to the selected safety-backup directory.
//! Chỉ copy state project hiện có vào thư mục safety backup đã chọn.

use std::path::Path;

use super::error::PreserveStateError;
use crate::commands::safe_filesystem::copy_regular_file_no_follow;
use crate::commands::store::backup_storage::{
    copy_dir, ensure_empty_destination_directory, prepare_destination_root,
};

pub(in crate::commands::store::restore) fn preserve_current_state(
    database: Option<&Path>,
    virtual_store: Option<&Path>,
    safety_backup: &Path,
) -> Result<(), PreserveStateError> {
    let root = prepare_destination_root(safety_backup).map_err(|source| {
        PreserveStateError::PrepareRoot {
            path: safety_backup.to_path_buf(),
            source,
        }
    })?;
    ensure_empty_destination_directory(&root).map_err(|source| {
        PreserveStateError::ValidateEmptyRoot {
            path: root.clone(),
            source,
        }
    })?;
    if let Some(virtual_store) = virtual_store {
        copy_dir(virtual_store, &root.join("vstore")).map_err(PreserveStateError::CopyVstore)?;
    }
    if let Some(database) = database {
        copy_regular_file_no_follow(database, &root.join("store.db")).map_err(|source| {
            PreserveStateError::CopyDatabase {
                path: database.to_path_buf(),
                source,
            }
        })?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
