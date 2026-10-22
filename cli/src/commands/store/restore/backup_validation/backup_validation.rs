//! Resolves only safe, present components from a restore source.
//! Phân giải các thành phần hiện có và an toàn trong nguồn restore.

use std::fs;
use std::path::{Path, PathBuf};

use super::error::BackupValidationError;
use crate::commands::safe_filesystem::canonical_directory;
use crate::commands::store::backup_storage::validate_present_tree;

#[derive(Debug)]
pub(in crate::commands::store::restore) struct ValidatedBackup {
    pub(in crate::commands::store::restore) root: PathBuf,
    pub(in crate::commands::store::restore) database: Option<PathBuf>,
    pub(in crate::commands::store::restore) virtual_store: Option<PathBuf>,
}

pub(in crate::commands::store::restore) fn validate_backup(
    path: &Path,
) -> Result<ValidatedBackup, BackupValidationError> {
    let root = canonical_directory(path).map_err(|source| BackupValidationError::ValidateRoot {
        path: path.to_path_buf(),
        source,
    })?;
    let database = existing_database(&root)?;
    let virtual_store = existing_vstore(&root)?;
    if database.is_none() && virtual_store.is_none() {
        return Err(BackupValidationError::EmptyBackup(path.to_path_buf()));
    }
    Ok(ValidatedBackup {
        root,
        database,
        virtual_store,
    })
}

fn existing_database(root: &Path) -> Result<Option<PathBuf>, BackupValidationError> {
    let path = root.join("store.db");
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source) => {
            return Err(BackupValidationError::InspectEntry { path, source });
        }
    };
    if metadata.file_type().is_symlink() {
        return Err(BackupValidationError::SymlinkEntry(path));
    }
    if !metadata.is_file() {
        return Err(BackupValidationError::DatabaseNotRegular(path));
    }
    let canonical =
        fs::canonicalize(&path).map_err(|source| BackupValidationError::CanonicalizeEntry {
            path: path.clone(),
            source,
        })?;
    if !canonical.starts_with(root) {
        return Err(BackupValidationError::EntryEscapesRoot {
            entry: path.clone(),
            target: canonical,
            root: root.to_path_buf(),
        });
    }
    let mut file = mgc_platform::open_regular_file_no_follow(&path).map_err(|source| {
        BackupValidationError::OpenDatabase {
            path: path.clone(),
            source,
        }
    })?;
    use std::io::Read;
    let mut probe = [0_u8; 1];
    file.read(&mut probe)
        .map_err(|source| BackupValidationError::InspectEntry {
            path: path.clone(),
            source,
        })?;
    Ok(Some(canonical))
}

fn existing_vstore(root: &Path) -> Result<Option<PathBuf>, BackupValidationError> {
    let path = root.join("vstore");
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source) => {
            return Err(BackupValidationError::InspectEntry { path, source });
        }
    };
    if metadata.file_type().is_symlink() {
        return Err(BackupValidationError::SymlinkEntry(path));
    }
    if !metadata.is_dir() {
        return Err(BackupValidationError::VstoreNotDirectory(path));
    }
    let canonical =
        fs::canonicalize(&path).map_err(|source| BackupValidationError::CanonicalizeEntry {
            path: path.clone(),
            source,
        })?;
    if !canonical.starts_with(root) {
        return Err(BackupValidationError::EntryEscapesRoot {
            entry: path.clone(),
            target: canonical,
            root: root.to_path_buf(),
        });
    }
    let _validated_tree =
        validate_present_tree(&path).map_err(BackupValidationError::ValidateVstore)?;
    Ok(Some(canonical))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
