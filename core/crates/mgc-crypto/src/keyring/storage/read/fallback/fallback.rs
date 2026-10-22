//! Fallback secure keyring reads — Đọc keyring bảo mật trên Fallback.
//! Reject non-regular key sources — Từ chối nguồn key không phải file thường.

use crate::CryptoResult;
use std::fs;
use std::path::Path;

pub(in crate::keyring::storage) fn read_keyring_contents(path: &Path) -> CryptoResult<String> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.file_type().is_file() {
        return Err(crate::keyring::error::KeyringError::NotRegularFile.into());
    }
    Ok(fs::read_to_string(path)?)
}
