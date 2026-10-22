//! Secure keyring file reads — Đọc file keyring bảo mật.
//! Select OS handles behind one read contract — Chọn handle OS sau một contract đọc.

use crate::CryptoResult;
use std::path::Path;

pub(in crate::keyring) fn read_keyring_contents(path: &Path) -> CryptoResult<String> {
    #[cfg(unix)]
    {
        super::unix::read_keyring_contents(path)
    }
    #[cfg(windows)]
    {
        return super::windows::read_keyring_contents(path);
    }
    #[cfg(not(any(unix, windows)))]
    {
        super::fallback::read_keyring_contents(path)
    }
}
