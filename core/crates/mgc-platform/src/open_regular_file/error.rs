//! Typed failures from the shared no-follow file opener.
//! Lỗi typed từ helper mở file no-follow dùng chung.

use std::io;

#[derive(Debug, thiserror::Error)]
pub enum OpenRegularFileError {
    #[error("path is a symbolic link")]
    Symlink,
    #[error("path is not a regular file")]
    NotRegular,
    #[error("opened path is not a regular file")]
    OpenedNotRegular,
    #[cfg(windows)]
    #[error("opened path is a reparse point")]
    ReparsePoint,
    #[cfg(not(any(unix, windows)))]
    #[error("secure no-follow opening is unsupported on this platform")]
    UnsupportedPlatform,
    #[error(transparent)]
    Io(#[from] io::Error),
}
