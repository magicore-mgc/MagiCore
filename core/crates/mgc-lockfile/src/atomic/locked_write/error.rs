//! Classifies failures from the project-lock-protected writer.
//! Phân loại lỗi của writer được bảo vệ bởi project lock.

/// Typed internal failures mapped to the stable crate-level write error.
/// Lỗi nội bộ có kiểu, được ánh xạ sang lỗi ghi cấp crate ổn định.
#[derive(Debug, thiserror::Error)]
pub(super) enum LockedWriteError {
    #[error("cannot create lock temp '{path}': {source}")]
    CreateTemp {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("cannot write lock temp '{path}': {source}")]
    WriteTemp {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("cannot fsync lock temp '{path}': {source}")]
    SyncTemp {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[cfg(windows)]
    #[error("refusing to write through a symlink temp path")]
    SymlinkTemp,
    #[error("cannot replace '{path}': {source} (old lock untouched)")]
    Replace {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("cannot fsync lock dir '{path}': {source}")]
    SyncDirectory {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

impl From<LockedWriteError> for crate::LockfileError {
    fn from(error: LockedWriteError) -> Self {
        Self::WriteFailed(error.to_string())
    }
}
