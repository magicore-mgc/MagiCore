//! Typed failures owned by the AI build lane.
//! Lỗi typed thuộc lane build AI.

use std::fmt;
use std::path::PathBuf;

/// Failures whose diagnostic and context belong to AI build selection/execution.
#[derive(Debug)]
pub(super) enum AiBuildError {
    FrameworkNotDetected { root: PathBuf },
    ToolchainMissing { tool: String },
    PythonBuildFailed { source: anyhow::Error },
}

impl fmt::Display for AiBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FrameworkNotDetected { root } => {
                write!(
                    formatter,
                    "No ai build framework detected in {}",
                    root.display()
                )
            }
            Self::ToolchainMissing { tool } => write!(
                formatter,
                "'{tool}' not found in PATH — install it first (mgc doctor check lists required toolchains)"
            ),
            Self::PythonBuildFailed { source } => write!(
                formatter,
                "Python build backend failed: {source} — MagiCore does not install Python build tools; provide a supported build backend before retrying"
            ),
        }
    }
}

impl std::error::Error for AiBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PythonBuildFailed { source } => Some(source.as_ref()),
            Self::FrameworkNotDetected { .. } | Self::ToolchainMissing { .. } => None,
        }
    }
}
