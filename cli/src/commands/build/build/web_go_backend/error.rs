//! Typed failures owned by the Web Go build lane.
//! Lỗi typed thuộc lane build Go của Web.

use std::fmt;

/// Go toolchain and compiler failures for the Web backend.
#[derive(Debug)]
pub(super) enum WebGoBuildError {
    ToolchainMissing { tool: &'static str },
    GoBuildFailed { source: anyhow::Error },
}

impl fmt::Display for WebGoBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToolchainMissing { tool } => write!(
                formatter,
                "'{tool}' not found in PATH — install it first (mgc doctor check lists required toolchains)"
            ),
            Self::GoBuildFailed { source } => write!(formatter, "go build ./... failed: {source}"),
        }
    }
}

impl std::error::Error for WebGoBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::GoBuildFailed { source } => Some(source.as_ref()),
            Self::ToolchainMissing { .. } => None,
        }
    }
}
