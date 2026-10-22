//! Typed filesystem and archive failures during Cargo layout materialization.
//! Lỗi filesystem và archive có kiểu khi materialize layout Cargo.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::protocols::crates) enum MaterializeError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Archive(#[from] MgError),
}

pub(in crate::protocols::crates) type MaterializeResult<T> = Result<T, MaterializeError>;

impl From<MaterializeError> for MgError {
    fn from(error: MaterializeError) -> Self {
        match error {
            MaterializeError::Io(error) => Self::Io(error),
            MaterializeError::Archive(error) => error,
        }
    }
}
