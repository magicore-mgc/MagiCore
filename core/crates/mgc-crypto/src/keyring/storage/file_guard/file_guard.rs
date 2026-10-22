//! Existing keyring and backup path checks — Kiểm tra path keyring và backup hiện có.
//! Refuse symlink and non-regular replacement targets — Chặn symlink và đích replace không regular.

use crate::CryptoResult;
use crate::keyring::storage::read::read_keyring_contents;
use std::fs;
use std::path::Path;

pub(in crate::keyring) fn read_existing_for_backup(path: &Path) -> CryptoResult<Option<String>> {
    match fs::symlink_metadata(path) {
        Ok(_) => read_keyring_contents(path).map(Some),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.into()),
    }
}

pub(in crate::keyring) fn reject_non_regular_existing_path(path: &Path) -> CryptoResult<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => Err(
            crate::keyring::error::KeyringError::RefuseNonRegularPath(path.display().to_string())
                .into(),
        ),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}
