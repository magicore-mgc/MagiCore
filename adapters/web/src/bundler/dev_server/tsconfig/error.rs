//! Typed tsconfig read failures — lỗi đọc tsconfig có kiểu.
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub(in crate::bundler::dev_server::implementation) enum TsconfigDigestError {
    CannotAccess { path: PathBuf, source: io::Error },
    CannotRead { path: PathBuf, source: io::Error },
}

impl fmt::Display for TsconfigDigestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CannotAccess { path, source } => write!(
                formatter,
                "tsconfig.json exists but cannot be accessed ({}): {source} — refusing to compile with unknown semantics; fix permissions and reload",
                path.display()
            ),
            Self::CannotRead { path, source } => write!(
                formatter,
                "tsconfig.json cannot be read ({}): {source} — refusing to compile with unknown semantics; fix permissions and reload",
                path.display()
            ),
        }
    }
}

impl std::error::Error for TsconfigDigestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CannotAccess { source, .. } | Self::CannotRead { source, .. } => Some(source),
        }
    }
}
