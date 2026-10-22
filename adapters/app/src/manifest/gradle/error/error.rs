//! Typed local errors for Gradle manifest operations.
//! Lỗi typed cục bộ cho các thao tác manifest Gradle.

use mgc_types::MgError;

/// Local classifications preserve the public `MgError` contract when converted back.
/// Phân loại cục bộ giữ hợp đồng `MgError` công khai khi chuyển đổi ngược.
#[derive(Debug, thiserror::Error)]
pub(in crate::manifest::gradle) enum GradleManifestError {
    #[error(transparent)]
    Source(#[from] MgError),
    #[error("{0}")]
    Other(String),
    #[error("{core} core does not support '{capability}' yet: {guidance}")]
    Unsupported {
        core: &'static str,
        capability: &'static str,
        guidance: String,
    },
}

impl GradleManifestError {
    /// Restore the original public error variant and source at the API boundary.
    /// Khôi phục variant lỗi và source công khai ban đầu tại API boundary.
    pub(in crate::manifest::gradle) fn into_mg_error(self) -> MgError {
        match self {
            Self::Source(error) => error,
            Self::Other(message) => MgError::Other(message),
            Self::Unsupported {
                core,
                capability,
                guidance,
            } => MgError::Unsupported {
                core,
                capability,
                guidance,
            },
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
