//! Typed simple-index failures with stable HTTP status mapping.
//! Lỗi simple-index có kiểu, giữ nguyên ánh xạ HTTP status.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::{error::Error, fmt};

#[derive(Debug)]
pub(in crate::pypi) enum SimpleIndexError {
    RequestStatus(StatusCode),
    StorageRead(anyhow::Error),
    PackageNotFound,
}

impl SimpleIndexError {
    pub(super) fn status_code(&self) -> StatusCode {
        match self {
            Self::RequestStatus(status) => *status,
            Self::StorageRead(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PackageNotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl From<StatusCode> for SimpleIndexError {
    fn from(status: StatusCode) -> Self {
        Self::RequestStatus(status)
    }
}

impl fmt::Display for SimpleIndexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestStatus(status) => {
                write!(f, "PyPI simple-index request failed with {status}")
            }
            Self::StorageRead(_) => f.write_str("PyPI simple-index storage read failed"),
            Self::PackageNotFound => f.write_str("PyPI package has no published files"),
        }
    }
}

impl Error for SimpleIndexError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::StorageRead(source) => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl IntoResponse for SimpleIndexError {
    fn into_response(self) -> Response {
        self.status_code().into_response()
    }
}
