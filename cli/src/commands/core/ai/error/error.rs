//! Typed failures owned by the AI core command boundary.
//! Lỗi có kiểu thuộc boundary lệnh AI core.

use std::error::Error as StdError;
use std::fmt;
use std::path::PathBuf;

/// AI command failures retain the established diagnostic text and source chain.
/// Lỗi lệnh AI giữ nguyên thông báo hiện có và chuỗi nguyên nhân.
#[derive(Debug)]
pub(in crate::commands::core::ai) enum AiCoreError {
    ProjectNotDetected,
    FrameworkNotDetected {
        root: PathBuf,
    },
    NativeDependencyLifecycleUnavailable {
        operation: String,
    },
    PythonRuntimeFailed {
        launcher: String,
        source: anyhow::Error,
    },
}

impl AiCoreError {
    pub(in crate::commands::core::ai) fn project_not_detected() -> Self {
        Self::ProjectNotDetected
    }

    pub(in crate::commands::core::ai) fn framework_not_detected(root: &std::path::Path) -> Self {
        Self::FrameworkNotDetected {
            root: root.to_path_buf(),
        }
    }

    pub(in crate::commands::core::ai) fn native_dependency_lifecycle_unavailable(
        operation: &str,
    ) -> Self {
        Self::NativeDependencyLifecycleUnavailable {
            operation: operation.to_owned(),
        }
    }

    pub(in crate::commands::core::ai) fn python_runtime_failed(
        launcher: &str,
        source: anyhow::Error,
    ) -> Self {
        Self::PythonRuntimeFailed {
            launcher: launcher.to_owned(),
            source,
        }
    }
}

impl fmt::Display for AiCoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProjectNotDetected => formatter.write_str(
                "Cannot detect an ai project here (missing mgc.toml [ai] framework / pyproject [tool.magicore] framework).",
            ),
            Self::FrameworkNotDetected { root } => {
                write!(formatter, "No ai framework detected in {}", root.display())
            }
            Self::NativeDependencyLifecycleUnavailable { operation } => write!(
                formatter,
                "`ai` {operation} for `python` is unavailable: MagiCore does not yet own the complete native dependency lifecycle for this lane. No external package manager was invoked; use a supported native lane or wait for native resolver, lock, fetch, verify, store, and materializer support."
            ),
            Self::PythonRuntimeFailed { launcher, source } => {
                write!(formatter, "Python launcher '{launcher}' failed: {source}")
            }
        }
    }
}

impl StdError for AiCoreError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::PythonRuntimeFailed { source, .. } => Some(source.as_ref()),
            Self::ProjectNotDetected
            | Self::FrameworkNotDetected { .. }
            | Self::NativeDependencyLifecycleUnavailable { .. } => None,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
