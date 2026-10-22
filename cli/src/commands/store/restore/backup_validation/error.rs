//! Typed failures while validating a restore source.
//! Lỗi có type khi xác thực nguồn restore.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::store::restore) enum BackupValidationError {
    #[error("backup contains neither a regular store.db nor a vstore directory: {0}")]
    EmptyBackup(PathBuf),
    #[error("inspect backup entry {path}: {source}")]
    InspectEntry {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("backup entry is a symlink: {0}")]
    SymlinkEntry(PathBuf),
    #[error("backup database is not a regular file: {0}")]
    DatabaseNotRegular(PathBuf),
    #[error("backup virtual store is not a real directory: {0}")]
    VstoreNotDirectory(PathBuf),
    #[error("backup entry escapes backup root: entry={entry}, target={target}, root={root}")]
    EntryEscapesRoot {
        entry: PathBuf,
        target: PathBuf,
        root: PathBuf,
    },
    #[error("open backup database without following symlinks {path}: {source}")]
    OpenDatabase {
        path: PathBuf,
        #[source]
        source: mgc_platform::OpenRegularFileError,
    },
    #[error("validate backup directory {path}: {source}")]
    ValidateRoot {
        path: PathBuf,
        #[source]
        source: crate::commands::safe_filesystem::SafeFilesystemError,
    },
    #[error("validate backup virtual-store tree: {0}")]
    ValidateVstore(#[source] anyhow::Error),
    #[error("canonicalize backup entry {path}: {source}")]
    CanonicalizeEntry {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
