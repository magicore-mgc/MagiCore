//! Classifies `.magicore` directory and guard-path safety failures.
//! Phân loại lỗi an toàn directory `.magicore` và guard path.

/// Typed path failures mapped to the existing lockfile write-error boundary.
/// Lỗi path có kiểu, ánh xạ về boundary write error hiện tại.
#[derive(Debug, thiserror::Error)]
pub(in crate::project_lock) enum PathSafetyError {
    #[error("cannot create dir '{path}': {source}")]
    CreateDirectory {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("cannot stat dir '{path}': {source}")]
    StatDirectory {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("refusing link-swapped dir '{path}'")]
    LinkSwappedDirectory { path: String },
    #[error("path '{path}' is not a directory")]
    NotDirectory { path: String },
    #[error("refusing symlinked lock guard '{path}'")]
    SymlinkGuard { path: String },
    #[cfg(windows)]
    #[error("refusing reparse-point lock guard '{path}'")]
    ReparseGuard { path: String },
}

impl From<PathSafetyError> for crate::LockfileError {
    fn from(error: PathSafetyError) -> Self {
        Self::WriteFailed(error.to_string())
    }
}
