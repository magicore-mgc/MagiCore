//! Typed legacy-upload failures with stable HTTP status mapping.
//! Lỗi upload legacy có kiểu, giữ nguyên ánh xạ HTTP status.

use axum::{
    extract::multipart::MultipartError,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::{error::Error, fmt};

#[derive(Debug)]
pub(in crate::pypi) enum UploadLegacyError {
    RequestStatus(StatusCode),
    Multipart(MultipartError),
    Storage(anyhow::Error),
    Signing(anyhow::Error),
    DigestMismatch,
}

impl UploadLegacyError {
    pub(super) fn status_code(&self) -> StatusCode {
        match self {
            Self::RequestStatus(status) => *status,
            Self::Multipart(_) | Self::DigestMismatch => StatusCode::BAD_REQUEST,
            Self::Storage(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Signing(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl From<StatusCode> for UploadLegacyError {
    fn from(status: StatusCode) -> Self {
        Self::RequestStatus(status)
    }
}

impl fmt::Display for UploadLegacyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestStatus(status) => write!(f, "PyPI upload request failed with {status}"),
            Self::Multipart(_) => f.write_str("PyPI multipart upload could not be read"),
            Self::Storage(_) => f.write_str("PyPI upload storage operation failed"),
            Self::Signing(_) => f.write_str("PyPI trusted-publisher signing failed"),
            Self::DigestMismatch => f.write_str("PyPI upload SHA-256 digest did not match"),
        }
    }
}

impl Error for UploadLegacyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Multipart(source) => Some(source),
            Self::Storage(source) | Self::Signing(source) => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl IntoResponse for UploadLegacyError {
    fn into_response(self) -> Response {
        self.status_code().into_response()
    }
}
