//! Keep typed package-graph failures until the shared diagnostic boundary.
//! Giữ lỗi package graph có kiểu tới boundary diagnostic dùng chung.

use mgc_types::MgError;

/// Typed failures for root, registry, and SDK dependency graph checks.
/// Lỗi có kiểu khi kiểm tra graph dependency root, registry và SDK.
#[derive(Debug, thiserror::Error)]
pub(in crate::install::flutter_package_config::package_graph) enum FlutterPackageGraphError {
    #[error("read Flutter pubspec for package graph: {0}")]
    ReadRootPubspec(#[source] std::io::Error),
    #[error("parse Flutter pubspec for package graph: {0}")]
    ParseRootPubspec(#[source] serde_yaml::Error),
    #[error("Flutter pubspec has no valid package name")]
    InvalidRootName,
    #[error("Flutter root package graph dependency '{0}' is not resolved")]
    UnresolvedRootDependency(String),
    #[error("Flutter resolved graph contains the root project name '{0}'")]
    ResolvedGraphContainsRoot(String),
    #[error("Flutter SDK packages are required but no Flutter SDK root is available")]
    MissingSdkRoot,
    #[error("read Flutter SDK package graph manifest '{path}': {source}")]
    ReadSdkManifest {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("parse Flutter SDK package graph manifest '{path}': {source}")]
    ParseSdkManifest {
        path: String,
        #[source]
        source: serde_yaml::Error,
    },
    #[error("Flutter package graph contains duplicate package name '{0}'")]
    DuplicatePackage(String),
    #[error("Flutter SDK package graph identity mismatch for '{0}'")]
    SdkIdentityMismatch(String),
    #[error("Flutter SDK package '{0}' dependencies must be a mapping")]
    SdkDependenciesNotMapping(String),
    #[error("Flutter SDK dependency name must be a string")]
    NonStringSdkDependency,
    #[error("Flutter package graph edge '{package} -> {dependency}' has no resolved package")]
    UnresolvedPackageEdge { package: String, dependency: String },
    #[error("Flutter pubspec '{0}' must be a mapping")]
    RootDependencySectionNotMapping(String),
    #[error("Flutter pubspec '{0}' has a non-string name")]
    NonStringRootDependencyName(String),
    #[error(transparent)]
    Shared(#[from] MgError),
}

impl From<FlutterPackageGraphError> for MgError {
    fn from(error: FlutterPackageGraphError) -> Self {
        let message = error.to_string();
        match error {
            FlutterPackageGraphError::InvalidRootName
            | FlutterPackageGraphError::UnresolvedRootDependency(_)
            | FlutterPackageGraphError::ResolvedGraphContainsRoot(_)
            | FlutterPackageGraphError::DuplicatePackage(_)
            | FlutterPackageGraphError::SdkIdentityMismatch(_)
            | FlutterPackageGraphError::SdkDependenciesNotMapping(_)
            | FlutterPackageGraphError::NonStringSdkDependency
            | FlutterPackageGraphError::UnresolvedPackageEdge { .. }
            | FlutterPackageGraphError::RootDependencySectionNotMapping(_)
            | FlutterPackageGraphError::NonStringRootDependencyName(_) => Self::Integrity(message),
            FlutterPackageGraphError::Shared(shared) => shared,
            FlutterPackageGraphError::MissingSdkRoot => Self::Other(message),
            source_error @ (FlutterPackageGraphError::ReadRootPubspec(_)
            | FlutterPackageGraphError::ParseRootPubspec(_)
            | FlutterPackageGraphError::ReadSdkManifest { .. }
            | FlutterPackageGraphError::ParseSdkManifest { .. }) => {
                Self::Anyhow(source_error.into())
            }
        }
    }
}
