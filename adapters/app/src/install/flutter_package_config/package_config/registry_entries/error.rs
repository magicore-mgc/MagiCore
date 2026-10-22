//! Typed failures for materialized pub.dev package rows.
//! Lỗi có kiểu cho row package pub.dev đã materialize.

use mgc_types::MgError;

/// Local error boundary for registry package identity and manifest checks.
/// Ranh giới lỗi cục bộ cho danh tính package registry và kiểm tra manifest.
#[derive(Debug, thiserror::Error)]
pub(super) enum FlutterRegistryEntriesError {
    #[error("Flutter graph contains duplicate package name '{0}'")]
    DuplicateName(String),
    #[error("read materialized Flutter package manifest '{path}': {source}")]
    ReadManifest {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("parse materialized Flutter package manifest '{path}': {source}")]
    ParseManifest {
        path: String,
        #[source]
        source: serde_yaml::Error,
    },
    #[error("materialized Flutter package '{0}' has no valid pubspec name")]
    MissingManifestName(String),
    #[error("Flutter archive identity mismatch: graph says '{graph}', archive says '{archive}'")]
    ArchiveIdentity { graph: String, archive: String },
    #[error("canonicalize Flutter package root: {0}")]
    CanonicalizeRoot(#[source] std::io::Error),
    #[error("cannot encode Flutter package root URI")]
    EncodePackageRoot,
}

impl From<FlutterRegistryEntriesError> for MgError {
    fn from(error: FlutterRegistryEntriesError) -> Self {
        let message = error.to_string();
        match error {
            FlutterRegistryEntriesError::DuplicateName(_)
            | FlutterRegistryEntriesError::MissingManifestName(_)
            | FlutterRegistryEntriesError::ArchiveIdentity { .. } => Self::Integrity(message),
            FlutterRegistryEntriesError::ReadManifest { .. }
            | FlutterRegistryEntriesError::ParseManifest { .. }
            | FlutterRegistryEntriesError::CanonicalizeRoot(_)
            | FlutterRegistryEntriesError::EncodePackageRoot => Self::Other(message),
        }
    }
}
