//! Preserves CAS-specific messages while translating platform open failures.
//! Giữ message riêng của CAS khi chuyển lỗi mở file từ platform.

use std::io;
use std::path::Path;

use crate::cas::store::StoreError;
use mgc_platform::OpenRegularFileError;

#[derive(Debug, thiserror::Error)]
pub(super) enum OpenFileError {
    #[error("{0}")]
    Symlink(&'static str),
    #[error("path is not a regular file")]
    NotRegular,
    #[cfg(windows)]
    #[error("path is a reparse point, refusing to verify")]
    ReparsePoint,
    #[cfg(not(any(unix, windows)))]
    #[error("secure file opening is unsupported on this platform")]
    UnsupportedPlatform,
    #[error(transparent)]
    Io(#[from] io::Error),
}

impl OpenFileError {
    pub(super) fn from_platform(
        error: OpenRegularFileError,
        symlink_message: &'static str,
    ) -> Self {
        match error {
            OpenRegularFileError::Symlink => Self::Symlink(symlink_message),
            OpenRegularFileError::NotRegular | OpenRegularFileError::OpenedNotRegular => {
                Self::NotRegular
            }
            #[cfg(windows)]
            OpenRegularFileError::ReparsePoint => Self::ReparsePoint,
            #[cfg(not(any(unix, windows)))]
            OpenRegularFileError::UnsupportedPlatform => Self::UnsupportedPlatform,
            OpenRegularFileError::Io(source) => Self::Io(source),
        }
    }

    pub(super) fn into_store_error(self, path: &Path) -> StoreError {
        StoreError::Io {
            path: path.to_path_buf(),
            msg: self.to_string(),
        }
    }
}
