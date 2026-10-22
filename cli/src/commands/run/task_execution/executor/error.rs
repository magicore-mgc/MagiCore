//! Fail-closed diagnostics for resolving a project-owned task executable.
//! Lỗi đóng khi phân giải executable của task nằm ngoài quyền sở hữu project.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(super) enum TaskBinaryError {
    #[error("failed to inspect project task binary path '{path}': {source}")]
    Inspect {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("project task binary path '{path}' resolves outside project '{root}'")]
    OutsideProject { path: PathBuf, root: PathBuf },
    #[error("project task binary directory '{path}' is not a directory")]
    NotDirectory { path: PathBuf },
    #[error("project task binary '{path}' is not a regular file")]
    NotFile { path: PathBuf },
}
