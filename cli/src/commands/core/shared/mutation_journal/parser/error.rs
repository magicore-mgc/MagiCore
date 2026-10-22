//! Source-preserving mutation journal parse failures.
//! Lỗi parse journal giữ nguyên error source.

use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum MutationJournalParseError {
    Malformed {
        path: PathBuf,
        source: serde_json::Error,
    },
    UnsupportedSchema {
        version: String,
        expected: u32,
        path: PathBuf,
    },
    InvalidFields {
        path: PathBuf,
        source: serde_json::Error,
    },
}

impl fmt::Display for MutationJournalParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Malformed { path, source } | Self::InvalidFields { path, source } => write!(
                formatter,
                "corrupt mutation journal '{}': {source:#} (artifacts preserved — refusing to guess; inspect or delete it manually)",
                path.display()
            ),
            Self::UnsupportedSchema {
                version,
                expected,
                path,
            } => write!(
                formatter,
                "unsupported mutation journal schema v{version} in '{}' (this binary reads v{expected}; artifacts preserved — inspect or delete it manually)",
                path.display()
            ),
        }
    }
}

impl std::error::Error for MutationJournalParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Malformed { source, .. } | Self::InvalidFields { source, .. } => Some(source),
            Self::UnsupportedSchema { .. } => None,
        }
    }
}
