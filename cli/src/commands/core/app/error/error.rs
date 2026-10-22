//! App core typed errors and stable diagnostics.
//! Lỗi có kiểu và diagnostic ổn định của App core.

use std::error::Error as StdError;
use std::fmt;
use std::path::{Path, PathBuf};

/// App-specific identity, support and install-argument failures.
/// Lỗi identity, support và tham số install riêng của App.
#[derive(Debug)]
pub(in crate::commands::core::app) enum AppCoreError {
    ProjectNotDetected,
    LanguageNotDetected {
        root: PathBuf,
    },
    ManifestHint {
        verb: String,
        language: String,
        file: String,
    },
    NativeLifecycleUnavailable {
        ecosystem: String,
        operation: String,
    },
}

impl AppCoreError {
    pub(in crate::commands::core::app) fn project_not_detected() -> Self {
        Self::ProjectNotDetected
    }

    pub(in crate::commands::core::app) fn language_not_detected(root: &Path) -> Self {
        Self::LanguageNotDetected {
            root: root.to_path_buf(),
        }
    }

    pub(in crate::commands::core::app) fn manifest_hint(
        verb: &str,
        language: &str,
        file: &str,
    ) -> Self {
        Self::ManifestHint {
            verb: verb.to_owned(),
            language: language.to_owned(),
            file: file.to_owned(),
        }
    }

    pub(super) fn native_lifecycle_unavailable(ecosystem: &str, operation: &str) -> Self {
        Self::NativeLifecycleUnavailable {
            ecosystem: ecosystem.to_owned(),
            operation: operation.to_owned(),
        }
    }
}

impl fmt::Display for AppCoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProjectNotDetected => formatter.write_str(
                "Cannot detect an app project here (missing mgc.toml [app] language / pubspec.yaml / build.gradle/.kts / Package.swift).",
            ),
            Self::LanguageNotDetected { root } => {
                write!(formatter, "No app language detected in {}", root.display())
            }
            Self::ManifestHint {
                verb,
                language,
                file,
            } => write!(
                formatter,
                "'{verb}' for {language:?} has no CLI passthrough — edit {file} then run `mgc install`."
            ),
            Self::NativeLifecycleUnavailable {
                ecosystem,
                operation,
            } => write!(
                formatter,
                "`app` {operation} for `{ecosystem}` is unavailable: MagiCore does not yet own the complete native dependency lifecycle for this lane. No external package manager was invoked; use a supported native lane or wait for native resolver, lock, fetch, verify, store, and materializer support."
            ),
        }
    }
}

impl StdError for AppCoreError {}

/// Build the typed project-identity failure for App command consumers.
/// Tạo lỗi identity project có kiểu cho consumer lệnh App.
pub(crate) fn project_not_detected_error() -> anyhow::Error {
    anyhow::Error::new(AppCoreError::project_not_detected())
}

/// Build the typed native-lifecycle-unavailable failure for App consumers.
/// Tạo lỗi lane lifecycle native chưa có cho consumer App.
pub(crate) fn native_lifecycle_error(ecosystem: &str, operation: &str) -> anyhow::Error {
    anyhow::Error::new(AppCoreError::native_lifecycle_unavailable(
        ecosystem, operation,
    ))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
