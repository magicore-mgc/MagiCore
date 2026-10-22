//! Windows secure keyring reads — Đọc keyring bảo mật trên Windows.
//! Reject Windows reparse-point key sources — Chặn nguồn key Windows là reparse point.

use crate::CryptoResult;
use std::fs;
use std::os::windows::fs::OpenOptionsExt;
use std::path::Path;

pub(in crate::keyring::storage) fn read_keyring_contents(path: &Path) -> CryptoResult<String> {
    use std::io::Read;

    // OPEN_REPARSE_POINT blocks a final reparse point — Chặn reparse point cuối path.
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    let mut options = fs::OpenOptions::new();
    options
        .read(true)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    let mut file = options.open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(crate::keyring::error::KeyringError::NotRegularNonReparseFile.into());
    }

    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
