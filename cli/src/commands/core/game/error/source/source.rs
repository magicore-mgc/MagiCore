use super::super::GameOptimizerError;
use std::error::Error as StdError;

impl StdError for GameOptimizerError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::NativeManifestInspect { source }
            | Self::HookManifestInspect { source }
            | Self::TemplatePublishFailed { source } => Some(source),
            Self::TemplateDestinationInspectFailed { source, .. } => Some(source),
            Self::ManifestSnapshotFailed { source } => Some(source),
            Self::ManifestRollbackFailed { source, .. } => Some(source.as_ref()),
            Self::StagingPathRejected { source, .. }
            | Self::StagingOperationFailed { source, .. }
            | Self::StagingCleanupFailed { source, .. }
            | Self::TemplateDestinationRejected { source, .. } => Some(source.as_ref()),
            Self::ProjectNotDetected { .. }
            | Self::UnsupportedEngine { .. }
            | Self::NativeManifestMissing
            | Self::NativeManifestNotRegular
            | Self::HookManifestMissing
            | Self::HookManifestNotRegular
            | Self::DependencySectionMissing
            | Self::ConflictingDependency { .. }
            | Self::ManifestRoundTripFailed => None,
            Self::TemplateDestinationNotDirectory { .. } => None,
        }
    }
}
