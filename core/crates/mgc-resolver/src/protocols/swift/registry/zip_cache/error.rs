//! Local typed errors for bounded ZIP-cache operations.
//! Lỗi có kiểu cục bộ cho thao tác cache ZIP có giới hạn.

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(super) enum SwiftZipCacheError {
    LockPoisoned,
}

impl Display for SwiftZipCacheError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LockPoisoned => {
                formatter.write_str("swift zip cache lock poisoned — refusing uncertain cache")
            }
        }
    }
}

impl std::error::Error for SwiftZipCacheError {}

impl From<SwiftZipCacheError> for mgc_types::MgError {
    fn from(error: SwiftZipCacheError) -> Self {
        Self::Other(error.to_string())
    }
}
