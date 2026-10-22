//! Typed failures for no-follow staged regular-file copy.
//! Lỗi có type cho copy file thường bằng staging và no-follow.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(crate) enum FileCopyError {
    #[error("open source file {path} without following links: {source}")]
    OpenSource {
        path: PathBuf,
        #[source]
        source: mgc_platform::OpenRegularFileError,
    },
    #[error("canonicalize destination parent {path}: {source}")]
    CanonicalizeParent {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("destination parent is not a directory: {0}")]
    ParentNotDirectory(PathBuf),
    #[error("destination file has no parent or filename: {0}")]
    InvalidDestination(PathBuf),
    #[error("inspect destination file {path}: {source}")]
    InspectDestination {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("refuse symlink destination file: {0}")]
    SymlinkDestination(PathBuf),
    #[error("destination is not a regular file: {0}")]
    NotRegularDestination(PathBuf),
    #[error("create staging file beside {destination}: {source}")]
    CreateStaging {
        destination: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("copy source {source_path} to staging for {destination}: {source}")]
    Copy {
        source_path: PathBuf,
        destination: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("set staging-file permissions for {destination}: {source}")]
    SetPermissions {
        destination: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("sync staging file for {destination}: {source}")]
    Sync {
        destination: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("atomically publish copied file to {destination}: {source}")]
    Publish {
        destination: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("allocate a unique staging path beside {0}")]
    ExhaustedStagingNames(PathBuf),
}
