//! Typed failures for the Game optimizer boundary.
//! Lỗi có kiểu tại boundary optimizer của Game core.

use std::io;
use std::path::{Path, PathBuf};

/// Refusals and filesystem failures produced by Bevy optimizer setup.
/// Lỗi từ chối và filesystem do bước cấu hình optimizer Bevy tạo ra.
#[derive(Debug)]
pub(in crate::commands::core::game) enum GameOptimizerError {
    ProjectNotDetected {
        root: PathBuf,
    },
    UnsupportedEngine {
        detected: String,
    },
    ManifestSnapshotFailed {
        source: io::Error,
    },
    ManifestRollbackFailed {
        operation: String,
        original_error: String,
        source: anyhow::Error,
    },
    StagingPathRejected {
        path: PathBuf,
        source: anyhow::Error,
    },
    StagingOperationFailed {
        operation: String,
        path: PathBuf,
        source: anyhow::Error,
    },
    StagingCleanupFailed {
        operation: String,
        path: PathBuf,
        original_error: Option<String>,
        source: anyhow::Error,
    },
    TemplateDestinationRejected {
        path: PathBuf,
        source: anyhow::Error,
    },
    TemplateDestinationInspectFailed {
        path: PathBuf,
        source: io::Error,
    },
    TemplateDestinationNotDirectory {
        path: PathBuf,
    },
    NativeManifestMissing,
    NativeManifestInspect {
        source: io::Error,
    },
    NativeManifestNotRegular,
    HookManifestMissing,
    HookManifestInspect {
        source: io::Error,
    },
    HookManifestNotRegular,
    DependencySectionMissing,
    ConflictingDependency {
        path: String,
    },
    TemplatePublishFailed {
        source: io::Error,
    },
    ManifestRoundTripFailed,
}

impl GameOptimizerError {
    pub(in crate::commands::core::game) fn project_not_detected(root: &Path) -> Self {
        Self::ProjectNotDetected {
            root: root.to_path_buf(),
        }
    }

    pub(in crate::commands::core::game) fn unsupported_engine(detected: &str) -> Self {
        Self::UnsupportedEngine {
            detected: detected.to_owned(),
        }
    }

    pub(in crate::commands::core::game) fn manifest_snapshot_failed(source: io::Error) -> Self {
        Self::ManifestSnapshotFailed { source }
    }

    pub(in crate::commands::core::game) fn manifest_rollback_failed(
        operation: &str,
        original_error: &anyhow::Error,
        source: anyhow::Error,
    ) -> Self {
        Self::ManifestRollbackFailed {
            operation: operation.to_owned(),
            original_error: format!("{original_error:#}"),
            source,
        }
    }

    pub(in crate::commands::core::game) fn staging_path_rejected(
        path: &Path,
        source: anyhow::Error,
    ) -> Self {
        Self::StagingPathRejected {
            path: path.to_path_buf(),
            source,
        }
    }

    pub(in crate::commands::core::game) fn staging_operation_failed(
        operation: &str,
        path: &Path,
        source: anyhow::Error,
    ) -> Self {
        Self::StagingOperationFailed {
            operation: operation.to_owned(),
            path: path.to_path_buf(),
            source,
        }
    }

    pub(in crate::commands::core::game) fn staging_cleanup_failed(
        operation: &str,
        path: &Path,
        original_error: Option<&anyhow::Error>,
        source: anyhow::Error,
    ) -> Self {
        Self::StagingCleanupFailed {
            operation: operation.to_owned(),
            path: path.to_path_buf(),
            original_error: original_error.map(|error| format!("{error:#}")),
            source,
        }
    }

    pub(in crate::commands::core::game) fn template_destination_rejected(
        path: &Path,
        source: anyhow::Error,
    ) -> Self {
        Self::TemplateDestinationRejected {
            path: path.to_path_buf(),
            source,
        }
    }

    pub(in crate::commands::core::game) fn template_destination_inspect_failed(
        path: &Path,
        source: io::Error,
    ) -> Self {
        Self::TemplateDestinationInspectFailed {
            path: path.to_path_buf(),
            source,
        }
    }

    pub(in crate::commands::core::game) fn template_destination_not_directory(path: &Path) -> Self {
        Self::TemplateDestinationNotDirectory {
            path: path.to_path_buf(),
        }
    }

    pub(in crate::commands::core::game) fn native_manifest_missing() -> Self {
        Self::NativeManifestMissing
    }

    pub(in crate::commands::core::game) fn native_manifest_inspect(source: io::Error) -> Self {
        Self::NativeManifestInspect { source }
    }

    pub(in crate::commands::core::game) fn native_manifest_not_regular() -> Self {
        Self::NativeManifestNotRegular
    }

    pub(in crate::commands::core::game) fn hook_manifest_missing() -> Self {
        Self::HookManifestMissing
    }

    pub(in crate::commands::core::game) fn hook_manifest_inspect(source: io::Error) -> Self {
        Self::HookManifestInspect { source }
    }

    pub(in crate::commands::core::game) fn hook_manifest_not_regular() -> Self {
        Self::HookManifestNotRegular
    }

    pub(in crate::commands::core::game) fn dependency_section_missing() -> Self {
        Self::DependencySectionMissing
    }

    pub(in crate::commands::core::game) fn conflicting_dependency(path: &str) -> Self {
        Self::ConflictingDependency {
            path: path.to_owned(),
        }
    }

    pub(in crate::commands::core::game) fn template_publish_failed(source: io::Error) -> Self {
        Self::TemplatePublishFailed { source }
    }

    pub(in crate::commands::core::game) fn manifest_round_trip_failed() -> Self {
        Self::ManifestRoundTripFailed
    }
}
