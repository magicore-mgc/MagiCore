//! Typed failures while preserving pre-restore state.
//! Lỗi có type khi bảo toàn state trước restore.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::store::restore) enum PreserveStateError {
    #[error("prepare safety-backup directory {path}: {source}")]
    PrepareRoot {
        path: PathBuf,
        #[source]
        source: crate::commands::store::backup_storage::CopyTreeError,
    },
    #[error("validate that safety-backup directory {path} is empty: {source}")]
    ValidateEmptyRoot {
        path: PathBuf,
        #[source]
        source: crate::commands::store::backup_storage::CopyTreeError,
    },
    #[error("preserve current virtual store: {0}")]
    CopyVstore(#[source] anyhow::Error),
    #[error("preserve current database {path}: {source}")]
    CopyDatabase {
        path: PathBuf,
        #[source]
        source: crate::commands::safe_filesystem::FileCopyError,
    },
}
