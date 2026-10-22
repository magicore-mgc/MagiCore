//! Typed errors local to the search cache.
//! Lỗi typed cục bộ của cache tìm kiếm.

use std::io;
use std::time::SystemTimeError;

/// Local classification for cache failures; public methods unwrap to original errors.
/// Phân loại lỗi nội bộ; public methods trả lại lỗi gốc ở boundary.
#[derive(Debug, thiserror::Error)]
pub(in crate::cache) enum SearchCacheError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Clock(#[from] SystemTimeError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
}

impl SearchCacheError {
    pub(in crate::cache) fn into_anyhow(self) -> anyhow::Error {
        match self {
            Self::Io(error) => error.into(),
            Self::Clock(error) => error.into(),
            Self::Json(error) => error.into(),
            Self::Sqlite(error) => error.into(),
        }
    }
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
