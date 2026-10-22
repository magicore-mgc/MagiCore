//! Typed failures for shared path validation and logical payload measurement.
//! Lỗi có type cho xác thực đường dẫn và đo payload logic dùng chung.

#[derive(Debug, thiserror::Error)]
pub(crate) enum SafeFilesystemError {
    #[error("canonicalize directory {path}: {source}")]
    CanonicalizeDirectory {
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
    #[error("directory is not a real directory: {path}")]
    NotDirectory { path: String },
    #[error("refuse symlinked directory: {path}")]
    SymlinkedDirectory { path: String },
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
    #[error("entry changed type during inspection: {path}")]
    EntryChangedType { path: String },
    #[error("path {path} resolves outside its expected boundary {boundary}")]
    PathEscapesBoundary { path: String, boundary: String },
    #[error("logical payload byte count overflow while reading {path}")]
    PayloadSizeOverflow { path: String },
}
