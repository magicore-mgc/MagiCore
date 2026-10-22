//! Typed failures owned by local Node executable resolution.
//! Lỗi typed thuộc khâu tìm executable Node cục bộ.

use std::env::JoinPathsError;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub(super) enum NodeCommandError {
    MissingLocalExecutable { name: String, project_root: PathBuf },
    JoinPaths { source: JoinPathsError },
}

impl fmt::Display for NodeCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLocalExecutable { name, project_root } => write!(
                formatter,
                "Missing local executable '{name}'. Run 'mgc install-web' in '{}'.",
                project_root.display()
            ),
            Self::JoinPaths { source } => write!(formatter, "{source}"),
        }
    }
}

impl std::error::Error for NodeCommandError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::JoinPaths { source } => Some(source),
            Self::MissingLocalExecutable { .. } => None,
        }
    }
}
