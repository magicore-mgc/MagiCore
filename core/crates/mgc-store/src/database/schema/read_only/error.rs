//! Typed failures for opening a database without schema migration.
//! Lỗi có type khi mở database mà không chạy schema migration.

#[derive(Debug, thiserror::Error)]
pub(super) enum ReadOnlyOpenError {
    #[error("inspect read-only database {path}: {source}")]
    InspectPath {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("refuse symlinked read-only database: {path}")]
    SymlinkedDatabase { path: String },
    #[error("read-only database is not a regular file: {path}")]
    NotRegularFile { path: String },
    #[error("canonicalize read-only database {path}: {source}")]
    CanonicalizePath {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("open read-only database {path}: {source}")]
    Open {
        path: String,
        #[source]
        source: rusqlite::Error,
    },
    #[error("read schema version from {path}: {source}")]
    ReadSchemaVersion {
        path: String,
        #[source]
        source: rusqlite::Error,
    },
    #[error("read-only access does not migrate schema {found}; supported schema is {supported}")]
    UnsupportedSchema { found: i64, supported: i64 },
    #[error("configure read-only database busy timeout at {path}: {source}")]
    BusyTimeout {
        path: String,
        #[source]
        source: rusqlite::Error,
    },
    #[error("initialize process-local database coordination for {path}: {source}")]
    WriterGate {
        path: String,
        #[source]
        source: anyhow::Error,
    },
}
