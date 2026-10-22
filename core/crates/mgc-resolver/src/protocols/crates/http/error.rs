//! Typed Crates.io HTTP failures before the shared resolver boundary.
//! Lỗi HTTP Crates.io có kiểu trước boundary resolver dùng chung.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::protocols::crates) enum HttpError {
    #[error("{0}")]
    Network(String),
}

pub(in crate::protocols::crates) type HttpResult<T> = Result<T, HttpError>;

impl From<HttpError> for MgError {
    fn from(error: HttpError) -> Self {
        match error {
            HttpError::Network(message) => Self::Network(message),
        }
    }
}
