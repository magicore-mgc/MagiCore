//! Typed failures while publishing a restored database.
//! Lỗi có type khi publish database đã restore.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::store::restore) enum DatabaseRestoreError {
    #[error("database destination has no parent or filename: {0}")]
    InvalidDestination(PathBuf),
    #[error("prepare database destination parent {path}: {source}")]
    PrepareParent {
        path: PathBuf,
        #[source]
        source: crate::commands::store::backup_storage::CopyTreeError,
    },
    #[error("copy restored database {source_path} to {destination}: {source}")]
    Copy {
        source_path: PathBuf,
        destination: PathBuf,
        #[source]
        source: crate::commands::safe_filesystem::FileCopyError,
    },
}
