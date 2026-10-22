//! Typed default backup directory failures.
//! Lỗi có type khi tạo path backup mặc định.
use std::path::PathBuf;
#[derive(Debug, thiserror::Error)]
pub(super) enum DefaultPathError {
    #[error("cannot resolve the current user's home directory")]
    NoHomeDirectory,
    #[error("inspect default backup root {path}: {source}")]
    Inspect {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("refuse symlink default backup root: {0}")]
    Symlink(PathBuf),
    #[error("default backup root is not a directory: {0}")]
    NotDirectory(PathBuf),
}
