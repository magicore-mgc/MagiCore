//! Typed path and measurement failures owned by candidate planning.
//! Lỗi path và phép đo có type thuộc owner lập kế hoạch candidate.

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::store::prune) enum CandidatePlanError {
    #[error(transparent)]
    Filesystem(#[from] crate::commands::safe_filesystem::SafeFilesystemError),
    #[error("invalid Web virtual-store package path for {package}: {path}")]
    InvalidCandidateComponent { package: String, path: String },
    #[error("logical payload total overflow before pruning package {package}")]
    PayloadTotalOverflow { package: String },
}
