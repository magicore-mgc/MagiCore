//! Typed failures for Flutter SDK package rows.
//! Lỗi có kiểu cho row package Flutter SDK.

use mgc_types::MgError;

/// Local error boundary for Flutter SDK package identity and filesystem checks.
/// Ranh giới lỗi cục bộ cho danh tính package Flutter SDK và kiểm tra filesystem.
#[derive(Debug, thiserror::Error)]
pub(super) enum FlutterSdkEntriesError {
    #[error(
        "Flutter SDK dependencies are declared but no Flutter SDK root was found; set FLUTTER_ROOT or install flutter on PATH"
    )]
    MissingSdkRoot,
    #[error("Flutter SDK package '{0}' conflicts with a registry package in the resolved graph")]
    RegistryConflict(String),
    #[error("inspect Flutter SDK package manifest '{path}': {source}")]
    InspectManifest {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("Flutter SDK package '{0}' has a linked or non-regular pubspec.yaml")]
    NonRegularManifest(String),
    #[error("read Flutter SDK package manifest '{path}': {source}")]
    ReadManifest {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("parse Flutter SDK package manifest '{path}': {source}")]
    ParseManifest {
        path: String,
        #[source]
        source: serde_yaml::Error,
    },
    #[error("Flutter SDK package identity mismatch for '{0}'")]
    IdentityMismatch(String),
    #[error("Flutter SDK package '{0}' has no regular lib directory")]
    MissingRegularLib(String),
    #[error("cannot encode Flutter SDK package URI")]
    EncodePackageUri,
    #[error(transparent)]
    Shared(#[from] MgError),
}

impl From<FlutterSdkEntriesError> for MgError {
    fn from(error: FlutterSdkEntriesError) -> Self {
        let message = error.to_string();
        match error {
            FlutterSdkEntriesError::RegistryConflict(_)
            | FlutterSdkEntriesError::NonRegularManifest(_)
            | FlutterSdkEntriesError::IdentityMismatch(_)
            | FlutterSdkEntriesError::MissingRegularLib(_) => Self::Integrity(message),
            FlutterSdkEntriesError::Shared(shared) => shared,
            FlutterSdkEntriesError::MissingSdkRoot
            | FlutterSdkEntriesError::InspectManifest { .. }
            | FlutterSdkEntriesError::ReadManifest { .. }
            | FlutterSdkEntriesError::ParseManifest { .. }
            | FlutterSdkEntriesError::EncodePackageUri => Self::Other(message),
        }
    }
}
