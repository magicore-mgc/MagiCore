use super::super::GameOptimizerError;
use std::fmt;

impl fmt::Display for GameOptimizerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProjectNotDetected { root } => write!(
                formatter,
                "game optimizer requires a detected native Bevy project; '{}' is not a detected game project",
                root.display()
            ),
            Self::UnsupportedEngine { detected } => write!(
                formatter,
                "game optimizer is only supported for native Bevy projects; detected '{detected}'"
            ),
            Self::ManifestSnapshotFailed { source } => write!(
                formatter,
                "cannot snapshot Cargo.toml before Bevy optimizer staging: {source}"
            ),
            Self::ManifestRollbackFailed {
                operation,
                original_error,
                source,
            } => write!(
                formatter,
                "optimizer {operation} failed ({original_error}); restoring Cargo.toml also failed: {source:#}"
            ),
            Self::StagingPathRejected { path, source } => write!(
                formatter,
                "refusing linked optimizer staging path '{}': {source:#}",
                path.display()
            ),
            Self::StagingOperationFailed {
                operation,
                path,
                source,
            } => write!(
                formatter,
                "cannot {operation} at '{}': {source:#}",
                path.display()
            ),
            Self::StagingCleanupFailed {
                operation,
                path,
                original_error,
                source,
            } => match original_error {
                Some(original_error) => write!(
                    formatter,
                    "{operation} failed ({original_error}); optimizer staging cleanup at '{}' also failed: {source:#}",
                    path.display()
                ),
                None => write!(
                    formatter,
                    "optimizer staging cleanup failed during {operation} at '{}': {source:#}",
                    path.display()
                ),
            },
            Self::TemplateDestinationRejected { path, source } => write!(
                formatter,
                "refusing linked optimizer template destination '{}': {source:#}",
                path.display()
            ),
            Self::TemplateDestinationInspectFailed { path, source } => write!(
                formatter,
                "cannot inspect optimizer template destination '{}': {source}",
                path.display()
            ),
            Self::TemplateDestinationNotDirectory { path } => write!(
                formatter,
                "optimizer template destination '{}' exists but is not a directory",
                path.display()
            ),
            Self::NativeManifestMissing => formatter.write_str(
                "native Bevy optimizer operations require Cargo.toml at the project root",
            ),
            Self::NativeManifestInspect { source } => {
                write!(formatter, "cannot inspect Bevy Cargo.toml: {source}")
            }
            Self::NativeManifestNotRegular => formatter.write_str(
                "native Bevy optimizer requires a regular non-symlink Cargo.toml",
            ),
            Self::HookManifestMissing => {
                formatter.write_str("game optimizer hook requires Cargo.toml")
            }
            Self::HookManifestInspect { source } => {
                write!(formatter, "cannot inspect game Cargo.toml: {source}")
            }
            Self::HookManifestNotRegular => {
                formatter.write_str("game optimizer hook requires a regular non-symlink Cargo.toml")
            }
            Self::DependencySectionMissing => {
                formatter.write_str("root Cargo.toml missing [dependencies]")
            }
            Self::ConflictingDependency { path } => write!(
                formatter,
                "game optimizer hook refused: Cargo.toml already has 'mgc-optimizer' with a different value ('{path}' != './optimizer') — resolve manually, never overwrite blindly"
            ),
            Self::TemplatePublishFailed { source } => {
                write!(formatter, "optimizer template publish failed: {source:#}")
            }
            Self::ManifestRoundTripFailed => formatter.write_str(
                "game optimizer hook write did not round-trip (re-read lacks mgc-optimizer = './optimizer') — refusing to report success",
            ),
        }
    }
}
