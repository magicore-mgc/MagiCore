//! Unix secure keyring reads — Đọc keyring bảo mật trên Unix.
//! Reject unsafe Unix key sources before decoding — Chặn nguồn key Unix không an toàn trước decode.

use crate::CryptoResult;
use std::fs;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::Path;

pub(in crate::keyring::storage) fn read_keyring_contents(path: &Path) -> CryptoResult<String> {
    use std::io::Read;

    let mut options = fs::OpenOptions::new();
    options
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    let mut file = options.open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(crate::keyring::error::KeyringError::NotRegularFile.into());
    }

    let mode = metadata.permissions().mode() & 0o7777;
    if mode & !0o600 != 0 {
        return Err(crate::keyring::error::KeyringError::InsecurePermissions {
            path: path.display().to_string(),
            mode,
        }
        .into());
    }

    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
