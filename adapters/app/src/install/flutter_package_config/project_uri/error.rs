//! Typed failures for Flutter project-root URI conversion.
//! Lỗi có kiểu khi chuyển URI root project Flutter.

use mgc_types::MgError;

/// Local error boundary for project-root path normalization and URI encoding.
/// Ranh giới lỗi cục bộ cho chuẩn hóa root và encode URI.
#[derive(Debug, thiserror::Error)]
pub(super) enum FlutterProjectUriError {
    #[error("cannot resolve relative Flutter project root without a current directory")]
    MissingCurrentDirectory,
    #[error("cannot encode Flutter project root URI")]
    EncodeProjectRoot,
    #[cfg(windows)]
    #[error("expand Windows Flutter project path: {0}")]
    ExpandWindowsPath(#[source] std::io::Error),
    #[cfg(windows)]
    #[error("read expanded Windows Flutter project path: {0}")]
    ReadExpandedWindowsPath(#[source] std::io::Error),
}

impl From<FlutterProjectUriError> for MgError {
    fn from(error: FlutterProjectUriError) -> Self {
        Self::Other(error.to_string())
    }
}
