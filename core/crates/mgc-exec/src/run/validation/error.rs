//! Typed failures raised by execution-argument validation.
//! Lỗi có kiểu do bước xác thực đối số thực thi phát sinh.

use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub(super) enum ArgumentValidationError {
    MissingProjectRoot {
        argument: String,
    },
    PathOutsideProject {
        argument: String,
        resolved: PathBuf,
        root: PathBuf,
    },
    CannotResolvePath {
        path: PathBuf,
        source: std::io::Error,
    },
}

impl fmt::Display for ArgumentValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingProjectRoot { argument } => write!(
                formatter,
                "Path argument rejected without project_root context: '{argument}'\n\
                 Cannot verify path safety without project boundary."
            ),
            Self::PathOutsideProject {
                argument,
                resolved,
                root,
            } => write!(
                formatter,
                "Path traversal detected: '{argument}' resolves to '{}' which is outside project root '{}'",
                resolved.display(),
                root.display()
            ),
            Self::CannotResolvePath { path, source } => {
                write!(
                    formatter,
                    "Failed to safely resolve path argument {path:?}: {source}"
                )
            }
        }
    }
}

impl std::error::Error for ArgumentValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CannotResolvePath { source, .. } => Some(source),
            Self::MissingProjectRoot { .. } | Self::PathOutsideProject { .. } => None,
        }
    }
}
