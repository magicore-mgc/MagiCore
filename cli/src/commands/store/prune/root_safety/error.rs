//! Typed failures owned by store-root and database-path validation.
//! Lỗi có type thuộc owner xác thực root store và path database.

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::store) enum RootSafetyError {
    #[error(transparent)]
    Filesystem(#[from] crate::commands::safe_filesystem::SafeFilesystemError),
    #[error("refuse symlinked store database: {path}")]
    SymlinkedDatabase { path: String },
    #[error("store database is not a regular file: {path}")]
    DatabaseNotRegularFile { path: String },
    #[error("canonicalize store database {path}: {source}")]
    CanonicalizeDatabase {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("store database {path} resolves outside its cache root {root}")]
    DatabaseEscapesCache { path: String, root: String },
    #[error("inspect store database {path}: {source}")]
    InspectDatabase {
        path: String,
        #[source]
        source: std::io::Error,
    },
}
