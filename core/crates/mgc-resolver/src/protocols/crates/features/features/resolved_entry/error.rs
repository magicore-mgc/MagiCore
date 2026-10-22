//! Typed integrity failures when projecting a resolved crate entry.
//! Lỗi toàn vẹn có kiểu khi chiếu entry crate đã resolve.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::protocols::crates) enum ResolvedEntryError {
    #[error("crates.io index entry has no checksum")]
    MissingChecksum,
}

pub(in crate::protocols::crates::features::features) type ResolvedEntryResult<T> =
    Result<T, ResolvedEntryError>;

impl From<ResolvedEntryError> for MgError {
    fn from(error: ResolvedEntryError) -> Self {
        match error {
            ResolvedEntryError::MissingChecksum => {
                Self::Integrity("crates.io index entry has no checksum".to_string())
            }
        }
    }
}
