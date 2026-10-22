//! Typed failures owned by native library build routes.
//! Lỗi typed thuộc các route build native của thư viện.

use std::fmt;
use std::path::PathBuf;

/// Compiler and unsupported-language failures specific to the Lib build lane.
#[derive(Debug)]
pub(super) enum LibBuildError {
    ToolchainMissing { tool: String },
    PythonBackendFailed { source: anyhow::Error },
    GoBuildFailed { source: anyhow::Error },
    DotnetBuildFailed { source: anyhow::Error },
    JavaBackendUnavailable { root: PathBuf },
}

impl fmt::Display for LibBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToolchainMissing { tool } => write!(
                formatter,
                "'{tool}' not found in PATH — install it first (mgc doctor check lists required toolchains)"
            ),
            Self::PythonBackendFailed { source } => write!(
                formatter,
                "Python build backend failed: {source} — MagiCore does not install Python build tools; provide a supported build backend before retrying"
            ),
            Self::GoBuildFailed { source } => write!(formatter, "go build ./... failed: {source}"),
            Self::DotnetBuildFailed { source } => {
                write!(
                    formatter,
                    "dotnet build failed: {source} — install the .NET SDK first"
                )
            }
            Self::JavaBackendUnavailable { root } => write!(
                formatter,
                "Java build descriptor found in '{}', but MagiCore has no native Java build backend yet; refusing to route the project through TypeScript or invoke Maven/Gradle",
                root.display()
            ),
        }
    }
}

impl std::error::Error for LibBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PythonBackendFailed { source }
            | Self::GoBuildFailed { source }
            | Self::DotnetBuildFailed { source } => Some(source.as_ref()),
            Self::ToolchainMissing { .. } | Self::JavaBackendUnavailable { .. } => None,
        }
    }
}
