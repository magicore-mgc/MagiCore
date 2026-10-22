//! Virtual-store cleanup errors — preserve the inspected path and I/O source.
//! Lỗi dọn virtual store — giữ path đã kiểm tra và source I/O.
#[derive(Debug, thiserror::Error)]
pub(super) enum VirtualStoreCleanupError {
    #[error("canonicalize project root {path}: {source}")]
    CanonicalizeProjectRoot {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("inspect directory {path}: {source}")]
    InspectDirectory {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("refuse symlinked directory root: {path}")]
    SymlinkedDirectoryRoot { path: String },
    #[error("expected a directory at {path}")]
    NotDirectory { path: String },
    #[error("canonicalize directory {path}: {source}")]
    CanonicalizeDirectory {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("directory {path} resolves outside its expected parent {parent}")]
    DirectoryEscapesParent { path: String, parent: String },
    #[error("read directory {path}: {source}")]
    ReadDirectory {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("read an entry in {path}: {source}")]
    ReadEntry {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("inspect entry {path}: {source}")]
    InspectEntry {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("package directory has a non-UTF-8 name and was preserved: {path}")]
    InvalidPackageName { path: String },
    #[error("symlinked package entry was preserved: {path}")]
    SymlinkedPackageEntry { path: String },
    #[error("entry changed type during inspection: {path}")]
    EntryChangedType { path: String },
    #[error("path {path} resolves outside its expected boundary {boundary}")]
    PathEscapesBoundary { path: String, boundary: String },
    #[error("logical payload byte count overflow while reading {path}")]
    PayloadSizeOverflow { path: String },
    #[error("logical payload byte count overflow before removing {path}")]
    ReportSizeOverflow { path: String },
    #[error("remove directory {path}: {source}")]
    RemoveDirectory {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

impl From<crate::commands::safe_filesystem::SafeFilesystemError> for VirtualStoreCleanupError {
    fn from(error: crate::commands::safe_filesystem::SafeFilesystemError) -> Self {
        use crate::commands::safe_filesystem::SafeFilesystemError as Shared;

        match error {
            Shared::CanonicalizeDirectory { path, source } => {
                Self::CanonicalizeDirectory { path, source }
            }
            Shared::InspectDirectory { path, source } => Self::InspectDirectory { path, source },
            Shared::NotDirectory { path } => Self::NotDirectory { path },
            Shared::SymlinkedDirectory { path } => Self::SymlinkedDirectoryRoot { path },
            Shared::DirectoryEscapesParent { path, parent } => {
                Self::DirectoryEscapesParent { path, parent }
            }
            Shared::ReadDirectory { path, source } => Self::ReadDirectory { path, source },
            Shared::ReadEntry { path, source } => Self::ReadEntry { path, source },
            Shared::InspectEntry { path, source } => Self::InspectEntry { path, source },
            Shared::EntryChangedType { path } => Self::EntryChangedType { path },
            Shared::PathEscapesBoundary { path, boundary } => {
                Self::PathEscapesBoundary { path, boundary }
            }
            Shared::PayloadSizeOverflow { path } => Self::PayloadSizeOverflow { path },
        }
    }
}
