//! Typed keyring policy failures — Lỗi policy keyring có kiểu.
//! Map local failures to stable public errors — Map lỗi local về lỗi public ổn định.

use crate::CryptoError;

#[derive(Debug, thiserror::Error)]
pub(super) enum KeyringError {
    #[error("keyring path must be in home directory")]
    PathOutsideHome,
    #[error("home directory not found")]
    HomeDirectoryMissing,
    #[error("key ID not found: {0}")]
    KeyNotFound(String),
    #[error("keyring path must resolve to a regular file")]
    NotRegularFile,
    // This is a Windows-only validation result — Đây là kết quả kiểm tra chỉ dùng trên Windows.
    #[cfg_attr(not(windows), allow(dead_code))]
    #[error("keyring path must resolve to a regular non-reparse file")]
    NotRegularNonReparseFile,
    #[error("system time precedes the UNIX epoch")]
    SystemTimeBeforeUnixEpoch,
    #[error(
        "keyring file '{path}' has insecure permissions {mode:04o}; \
         group/world access and executable bits are forbidden"
    )]
    InsecurePermissions { path: String, mode: u32 },
    #[error("refusing to replace non-regular or symlink keyring path '{0}'")]
    RefuseNonRegularPath(String),
    #[error("keyring destination must have a parent directory")]
    MissingParentDirectory,
    #[error("failed to generate temporary keyring name")]
    TemporaryNameGeneration,
    #[error("could not allocate a unique temporary keyring file")]
    TemporaryNameExhausted,
}

impl From<KeyringError> for CryptoError {
    fn from(error: KeyringError) -> Self {
        Self::KeyringFailed(error.to_string())
    }
}
