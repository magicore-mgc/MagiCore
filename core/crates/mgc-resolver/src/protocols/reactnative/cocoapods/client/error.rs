//! Typed CocoaPods client failures and shared-error classification.
//! Lỗi client CocoaPods có kiểu và phân loại về lỗi dùng chung.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::protocols::reactnative::cocoapods) enum CocoaPodsClientError {
    #[error("Podfile.lock checksum for {name} is not a sha1 hex digest: '{expected}'")]
    InvalidChecksum { name: String, expected: String },
    #[error("GET {url} failed: {source}")]
    RequestFailed {
        url: String,
        #[source]
        source: anyhow::Error,
    },
    #[error("read {url} body failed: {source}")]
    BodyReadFailed {
        url: String,
        #[source]
        source: anyhow::Error,
    },
    #[error("GET {url} returned {status}")]
    HttpStatus { url: String, status: String },
    #[error(
        "podspec sha1 mismatch for {name} {version}: Podfile.lock says {expected}, CDN spec is {actual} (fail-closed)"
    )]
    ChecksumMismatch {
        name: String,
        version: String,
        expected: String,
        actual: String,
    },
    #[error("parse podspec JSON for {name} {version} failed: {source}")]
    InvalidJson {
        name: String,
        version: String,
        #[source]
        source: serde_json::Error,
    },
}

impl CocoaPodsClientError {
    pub(in crate::protocols::reactnative::cocoapods) fn into_mg_error(self) -> MgError {
        let message = self.to_string();
        match self {
            Self::RequestFailed { .. } | Self::BodyReadFailed { .. } | Self::HttpStatus { .. } => {
                MgError::Network(message)
            }
            Self::InvalidChecksum { .. } | Self::ChecksumMismatch { .. } => {
                MgError::Integrity(message)
            }
            Self::InvalidJson { .. } => MgError::Other(message),
        }
    }
}

#[cfg(test)]
#[path = "test/error_mapping.rs"]
mod tests;
