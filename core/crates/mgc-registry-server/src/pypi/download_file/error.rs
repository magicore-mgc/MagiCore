//! Typed download failures with stable HTTP status mapping.
//! Lỗi download có kiểu, giữ nguyên ánh xạ HTTP status.

use axum::{
    http::{StatusCode, header::InvalidHeaderValue},
    response::{IntoResponse, Response},
};
use std::{error::Error, fmt};

#[derive(Debug)]
pub(in crate::pypi) enum DownloadFileError {
    RequestStatus(StatusCode),
    StorageRead(anyhow::Error),
    FileNotFound,
    InvalidHeaderValue(InvalidHeaderValue),
}

impl DownloadFileError {
    pub(super) fn status_code(&self) -> StatusCode {
        match self {
            Self::RequestStatus(status) => *status,
            Self::StorageRead(_) | Self::InvalidHeaderValue(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::FileNotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl From<StatusCode> for DownloadFileError {
    fn from(status: StatusCode) -> Self {
        Self::RequestStatus(status)
    }
}

impl fmt::Display for DownloadFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestStatus(status) => write!(f, "PyPI download request failed with {status}"),
            Self::StorageRead(_) => f.write_str("PyPI download storage read failed"),
            Self::FileNotFound => f.write_str("PyPI package file was not found"),
            Self::InvalidHeaderValue(_) => f.write_str("PyPI download response header is invalid"),
        }
    }
}

impl Error for DownloadFileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::StorageRead(source) => Some(source.as_ref()),
            Self::InvalidHeaderValue(source) => Some(source),
            _ => None,
        }
    }
}

impl IntoResponse for DownloadFileError {
    fn into_response(self) -> Response {
        self.status_code().into_response()
    }
}
