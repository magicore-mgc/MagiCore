//! Tree-backup preflight failures.
//! Lỗi preflight khi backup cây dependency.

use std::path::PathBuf;

/// Fail-closed errors from inspecting the live install tree.
/// Lỗi fail-closed khi kiểm tra cây install hiện tại.
#[derive(Debug)]
pub(super) enum TreeBackupError {
    InspectLiveTree {
        path: PathBuf,
        source: std::io::Error,
    },
    LiveTreeIsSymlink {
        path: PathBuf,
    },
    LiveTreeNotDirectory {
        path: PathBuf,
    },
    ReadLiveEntry {
        path: PathBuf,
        source: std::io::Error,
    },
}

impl std::fmt::Display for TreeBackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InspectLiveTree { path, source } => {
                write!(
                    f,
                    "cannot inspect existing install tree '{}': {source}",
                    path.display()
                )
            }
            Self::LiveTreeIsSymlink { path } => write!(
                f,
                "cannot stage install: existing '{}' is a symlink — refusing to replace or follow it",
                path.display()
            ),
            Self::LiveTreeNotDirectory { path } => write!(
                f,
                "cannot stage install: existing '{}' is not a directory",
                path.display()
            ),
            Self::ReadLiveEntry { path, source } => {
                write!(
                    f,
                    "cannot inspect entries in install tree '{}': {source}",
                    path.display()
                )
            }
        }
    }
}
