//! Typed failures produced while preparing a manifest delta.
//! Lỗi có kiểu khi chuẩn bị manifest delta.

use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub(in crate::commands::core::shared::implementation::implementation) enum ManifestDeltaError {
    InvalidPinnedRange(mgc_types::MgError),
}

impl From<mgc_types::MgError> for ManifestDeltaError {
    fn from(source: mgc_types::MgError) -> Self {
        Self::InvalidPinnedRange(source)
    }
}

impl fmt::Display for ManifestDeltaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPinnedRange(source) => fmt::Display::fmt(source, formatter),
        }
    }
}

impl StdError for ManifestDeltaError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::InvalidPinnedRange(source) => Some(source),
        }
    }
}
