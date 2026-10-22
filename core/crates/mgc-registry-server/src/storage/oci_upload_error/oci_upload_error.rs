//! Defines typed outcomes for resumable OCI upload append operations.
//! Định nghĩa lỗi typed cho thao tác append OCI upload có thể tiếp tục.

#[derive(Debug)]
pub enum OciUploadAppendError {
    SessionNotFound,
    OffsetMismatch,
    TooLarge,
    Storage(anyhow::Error),
}

impl From<anyhow::Error> for OciUploadAppendError {
    fn from(error: anyhow::Error) -> Self {
        Self::Storage(error)
    }
}

impl From<sqlx::Error> for OciUploadAppendError {
    fn from(error: sqlx::Error) -> Self {
        Self::Storage(error.into())
    }
}

impl From<std::io::Error> for OciUploadAppendError {
    fn from(error: std::io::Error) -> Self {
        Self::Storage(error.into())
    }
}

impl From<std::num::TryFromIntError> for OciUploadAppendError {
    fn from(error: std::num::TryFromIntError) -> Self {
        Self::Storage(error.into())
    }
}
