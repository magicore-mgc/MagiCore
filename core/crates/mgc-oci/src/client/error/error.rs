//! Typed registry status and digest failures.
//! Lỗi status registry và digest có kiểu.

#[derive(Debug)]
pub(in crate::client) enum OciClientError {
    CatalogFailed(String),
    BlobNotFound(String),
    HeadBlobFailed(String),
    GetBlobFailed(String),
    DigestMismatch { expected: String, got: String },
    PullManifestFailed(String),
    PushManifestFailed { status: String, body: String },
}

impl std::fmt::Display for OciClientError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CatalogFailed(status) => write!(formatter, "Catalog failed: {status}"),
            Self::BlobNotFound(digest) => write!(formatter, "Blob not found: {digest}"),
            Self::HeadBlobFailed(status) => write!(formatter, "HEAD blob failed: {status}"),
            Self::GetBlobFailed(status) => write!(formatter, "GET blob failed: {status}"),
            Self::DigestMismatch { expected, got } => {
                write!(formatter, "Digest mismatch: expected {expected}, got {got}")
            }
            Self::PullManifestFailed(status) => write!(formatter, "Pull manifest failed: {status}"),
            Self::PushManifestFailed { status, body } => {
                write!(formatter, "Push manifest failed: {status} - {body}")
            }
        }
    }
}

impl std::error::Error for OciClientError {}
