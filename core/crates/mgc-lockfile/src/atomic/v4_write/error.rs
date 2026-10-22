//! Classifies local v4 rewrite and atomic-publication failures.
//! Phân loại lỗi cục bộ khi rewrite và publish atomic v4.

/// Typed internal failures mapped to stable crate-level lockfile errors.
/// Lỗi nội bộ có kiểu, ánh xạ về lỗi lockfile cấp crate ổn định.
#[derive(Debug, thiserror::Error)]
pub(super) enum V4WriteError {
    #[error(
        "signed v4 mgc.lock cannot be rewritten without explicit re-signing; verify, remove the signature explicitly, then rewrite"
    )]
    SignedRewrite,
    #[error("mgc.lock path has no valid filename")]
    InvalidFilename,
    #[cfg(windows)]
    #[error("refusing to write through a symlink temp path")]
    SymlinkTemp,
    #[error("cannot create lock staging file: {source}")]
    CreateTemp {
        #[source]
        source: std::io::Error,
    },
    #[error("cannot write lock staging file: {source}")]
    WriteTemp {
        #[source]
        source: std::io::Error,
    },
    #[error("cannot sync lock staging file: {source}")]
    SyncTemp {
        #[source]
        source: std::io::Error,
    },
    #[error("cannot atomically replace mgc.lock: {source}")]
    Replace {
        #[source]
        source: std::io::Error,
    },
    #[error("cannot sync lock directory: {source}")]
    SyncDirectory {
        #[source]
        source: std::io::Error,
    },
}

impl From<V4WriteError> for crate::LockfileError {
    fn from(error: V4WriteError) -> Self {
        let signed_rewrite = matches!(&error, V4WriteError::SignedRewrite);
        let message = error.to_string();
        if signed_rewrite {
            Self::SignedLockMutation(message)
        } else {
            Self::WriteFailed(message)
        }
    }
}
