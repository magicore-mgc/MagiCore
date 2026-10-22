//! Typed errors for invalid manifest core hints.
//! Lỗi typed cho core hint manifest không hợp lệ.

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(super) enum CoreHintError {
    ConflictingManifestCores { cores: String },
    UnknownManifestCore { core: String },
}

impl Display for CoreHintError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConflictingManifestCores { cores } => write!(
                formatter,
                concat!(
                    "conflicting MagiCore core declarations in project manifests: {}. ",
                    "Set one authoritative `.mgc.core` marker or `mgc.toml` ecosystem ",
                    "before running core-aware commands."
                ),
                cores
            ),
            Self::UnknownManifestCore { core } => write!(
                formatter,
                "manifest declares unknown MagiCore core '{core}'; use `.mgc.core` or `mgc.toml` with a supported core"
            ),
        }
    }
}

impl std::error::Error for CoreHintError {}
