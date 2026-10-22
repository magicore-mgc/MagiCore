//! Classifies failures at the project-lock acquisition boundary.
//! Phân loại lỗi tại boundary acquire project lock.

/// Typed local acquisition failures mapped to stable crate errors.
/// Lỗi acquire cục bộ có kiểu, ánh xạ về lỗi crate ổn định.
#[derive(Debug, thiserror::Error)]
pub(super) enum ProjectLockError {
    #[error("cannot open lock guard '{path}': {source}")]
    OpenGuard {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("project lock held by a live process ('{path}'); refusing to reclaim")]
    Busy { path: String },
    #[error("cannot lock '{path}': {source}")]
    Lock {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

impl From<ProjectLockError> for crate::LockfileError {
    fn from(error: ProjectLockError) -> Self {
        let busy = matches!(&error, ProjectLockError::Busy { .. });
        let message = error.to_string();
        if busy {
            Self::LockBusy(message)
        } else {
            Self::WriteFailed(message)
        }
    }
}
