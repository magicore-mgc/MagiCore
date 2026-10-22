//! Typed failures while copying tree entries and publishing links.
//! Lỗi có type khi copy entry và publish link trong cây.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::store::backup_storage::copy_tree) enum TreeCopyError {
    #[error(transparent)]
    Destination(#[from] super::super::destination::DestinationError),
    #[error(transparent)]
    SourceValidation(#[from] super::super::source_validation::SourceValidationError),
    #[error(transparent)]
    FileCopy(#[from] crate::commands::safe_filesystem::FileCopyError),
    #[error("read source directory {path}: {source}")]
    ReadDirectory {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("read source directory entry in {path}: {source}")]
    ReadEntry {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("inspect source entry {path}: {source}")]
    InspectEntry {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("unsupported special source entry: {0}")]
    SpecialEntry(PathBuf),
    #[error("destination path conflicts with symlink copy: {0}")]
    LinkDestinationConflict(PathBuf),
    #[error("could not rebase symlink {link} to destination tree: {reason}")]
    RebaseLink { link: PathBuf, reason: &'static str },
    #[error("create destination symlink {path}: {source}")]
    CreateLink {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[cfg(not(any(unix, windows)))]
    #[error("symlink creation is unsupported on this platform")]
    UnsupportedPlatform,
}
