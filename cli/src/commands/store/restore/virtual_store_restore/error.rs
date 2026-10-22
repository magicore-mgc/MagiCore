//! Typed failures while replacing the project virtual store.
//! Lỗi có type khi thay virtual store project.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::store::restore) enum VirtualStoreRestoreError {
    #[error("virtual-store destination has no parent or filename: {0}")]
    InvalidDestination(PathBuf),
    #[error("prepare virtual-store destination parent {path}: {source}")]
    PrepareParent {
        path: PathBuf,
        #[source]
        source: crate::commands::store::backup_storage::CopyTreeError,
    },
    #[error("clear current virtual store before replacement: {0}")]
    Clear(#[source] anyhow::Error),
    #[error("copy validated virtual store into project: {0}")]
    Copy(#[source] anyhow::Error),
}
