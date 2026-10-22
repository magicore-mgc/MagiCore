//! Typed failure when an existing database cannot be opened without creation.
//! Lỗi có type khi không thể mở DB có sẵn mà không tạo file mới.

#[derive(Debug, thiserror::Error)]
pub(super) enum OpenExistingError {
    #[error("inspect existing store database {path}: {source}")]
    InspectPath {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("refuse symlinked existing store database: {path}")]
    SymlinkedDatabase { path: String },
    #[error("existing store database is not a regular file: {path}")]
    NotRegularFile { path: String },
    #[error("open existing store database {path} without creation: {source}")]
    Open {
        path: String,
        #[source]
        source: anyhow::Error,
    },
}
