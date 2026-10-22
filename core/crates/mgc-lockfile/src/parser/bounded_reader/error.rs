//! Bounded-reader failures and their stable public error mapping.
//! Lỗi bounded reader và ánh xạ về loại lỗi public ổn định.

use std::io;

#[derive(Debug, thiserror::Error)]
pub(super) enum BoundedReadError {
    #[error("{label} size limit is too large")]
    LimitOverflow { label: String },
    #[error("{label} size limit exceeds addressable memory")]
    LimitExceedsAddressSpace { label: String },
    #[error("{label} '{path}' must be a regular non-symlink file")]
    NotRegularPath { label: String, path: String },
    #[error("{label} too large: {size} bytes (max {max})")]
    PathTooLarge { label: String, size: u64, max: u64 },
    #[error("{label} '{path}' is not a regular file")]
    OpenedNotRegular { label: String, path: String },
    #[cfg(windows)]
    #[error("{label} '{path}' must not be a reparse point")]
    ReparsePoint { label: String, path: String },
    #[cfg(not(any(unix, windows)))]
    #[error("{label} secure file opening is unavailable on this platform")]
    UnsupportedPlatform { label: String },
    #[error("{label} size exceeds addressable memory")]
    OpenedSizeExceedsAddressSpace { label: String },
    #[error("{label} too large: more than {max} bytes")]
    ReadExceededLimit { label: String, max: u64 },
    #[error(transparent)]
    Io(#[from] io::Error),
}

impl From<BoundedReadError> for crate::LockfileError {
    fn from(error: BoundedReadError) -> Self {
        match error {
            BoundedReadError::Io(source) => Self::IoError(source),
            other => Self::ParseError(other.to_string()),
        }
    }
}
