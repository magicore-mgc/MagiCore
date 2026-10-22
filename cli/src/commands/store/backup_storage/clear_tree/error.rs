//! Typed clear-tree path and mutation failures.
//! Lỗi path và mutation có type của clear-tree.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(super) enum ClearTreeError {
    #[error("inspect clear root {path}: {source}")]
    InspectRoot {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("refuse symlink clear root: {0}")]
    SymlinkRoot(PathBuf),
    #[error("clear root is not a directory: {0}")]
    NotDirectory(PathBuf),
    #[error("read clear root {path}: {source}")]
    ReadDirectory {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("read entry under {path}: {source}")]
    ReadEntry {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("inspect entry {path}: {source}")]
    InspectEntry {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("remove symlink entry {path}: {source}")]
    RemoveLink {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("remove directory entry {path}: {source}")]
    RemoveDirectory {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("remove file entry {path}: {source}")]
    RemoveFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
