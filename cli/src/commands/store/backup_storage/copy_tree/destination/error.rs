//! Typed failures while resolving and creating destination directories.
//! Lỗi có type khi resolve và tạo directory đích.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::store::backup_storage::copy_tree) enum DestinationError {
    #[error("inspect destination root {path}: {source}")]
    InspectRoot {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("refuse symlink destination root: {0}")]
    SymlinkRoot(PathBuf),
    #[error("destination is not a directory: {0}")]
    NotDirectory(PathBuf),
    #[error("canonicalize destination root {path}: {source}")]
    CanonicalizeRoot {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("inspect destination directory {path}: {source}")]
    InspectDirectory {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("canonicalize destination directory {path}: {source}")]
    CanonicalizeDirectory {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("refuse symlink destination directory: {0}")]
    SymlinkDirectory(PathBuf),
    #[error("destination entry is not a real directory: {0}")]
    NotRealDirectory(PathBuf),
    #[error("destination directory already contains entries: {0}")]
    DestinationNotEmpty(PathBuf),
    #[error("destination directory {path} escapes expected root {root}")]
    DirectoryEscapesRoot { path: PathBuf, root: PathBuf },
    #[error("create destination directory {path}: {source}")]
    CreateDirectory {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
