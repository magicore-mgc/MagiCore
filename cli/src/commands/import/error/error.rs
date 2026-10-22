//! Import-local errors with preserved English messages and source chains.
//! Lỗi cục bộ của import, giữ thông báo tiếng Anh và chuỗi nguồn.

use std::fmt;
use std::path::PathBuf;

/// Failure categories owned by the import command.
/// Nhóm lỗi do lệnh import sở hữu.
#[derive(Debug)]
pub enum ImportError {
    ProjectConfig(anyhow::Error),
    LockfileRead {
        path: PathBuf,
        source: mgc_lockfile::LockfileError,
    },
    SignatureRead {
        path: PathBuf,
        source: mgc_lockfile::LockfileError,
    },
    UnsupportedCoreOwner {
        core: String,
    },
    UnsignedOptInRequired,
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProjectConfig(source) => write!(f, "{source}"),
            Self::LockfileRead { path, source } | Self::SignatureRead { path, source } => {
                write!(f, "cannot safely read '{}': {source}", path.display())
            }
            Self::UnsupportedCoreOwner { core } => {
                write!(f, "refusing to import lock for unknown core owner '{core}'")
            }
            Self::UnsignedOptInRequired => write!(
                f,
                "import refused: no default signing key and --allow-unsigned not given (unsigned lockfiles are never the default) — generate a key (`mgc trust`) or re-run with --allow-unsigned"
            ),
        }
    }
}

impl std::error::Error for ImportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectConfig(source) => Some(source.as_ref()),
            Self::LockfileRead { source, .. } | Self::SignatureRead { source, .. } => Some(source),
            Self::UnsupportedCoreOwner { .. } | Self::UnsignedOptInRequired => None,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
