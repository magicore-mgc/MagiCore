//! Typed project backup failures and operation context.
//! Lỗi backup project có type và context thao tác.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(super) enum BackupError {
    #[error("resolve default backup directory: {0}")]
    DefaultBackupPath(#[source] anyhow::Error),
    #[error("acquire project Web-store lock for backup: {source}")]
    AcquireProjectLock {
        #[source]
        source: anyhow::Error,
    },
    #[error("project Web-store root changed while acquiring the maintenance lock: {path}")]
    StoreRootChanged { path: PathBuf },
    #[error("backup destination overlaps project Web-store data: {destination}")]
    BackupDestinationOverlap { destination: PathBuf },
    #[error("backup destination changed during preparation: expected={expected}, actual={actual}")]
    BackupDestinationChanged { expected: PathBuf, actual: PathBuf },
    #[error("project has no Web database or virtual-store data to back up")]
    EmptyProjectStore,
    #[error("{operation}: {source}")]
    InspectProjectStore {
        operation: &'static str,
        #[source]
        source: anyhow::Error,
    },
    #[error("{operation}: {source}")]
    TreeOperation {
        operation: &'static str,
        #[source]
        source: anyhow::Error,
    },
    #[error("copy database {source_path} -> {destination_path}: {source}")]
    DatabaseCopy {
        source_path: PathBuf,
        destination_path: PathBuf,
        #[source]
        source: crate::commands::safe_filesystem::FileCopyError,
    },
    #[error("inspect backup entry {path}: {source}")]
    InspectBackupEntry {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("backup destination is a symlink or has an incompatible entry type: {0}")]
    InvalidBackupEntry(PathBuf),
    #[error("measure backup payload {path}: {source}")]
    MeasurePayload {
        path: PathBuf,
        #[source]
        source: anyhow::Error,
    },
}
