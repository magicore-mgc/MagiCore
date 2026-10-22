//! Typed failures for Dart package-config publication.
//! Lỗi có kiểu khi công bố package config Dart.

use mgc_types::MgError;

/// Local error boundary for `.dart_tool` validation and metadata publication.
/// Ranh giới lỗi cục bộ cho kiểm tra `.dart_tool` và công bố metadata.
#[derive(Debug, thiserror::Error)]
pub(super) enum FlutterPackageConfigError {
    #[error(
        "refusing to write Flutter package config through a linked or non-directory .dart_tool"
    )]
    UnsafeDartTool,
    #[error("inspect .dart_tool directory: {0}")]
    InspectDartTool(#[source] std::io::Error),
    #[error("Flutter package graph has no root package")]
    MissingGraphRoot,
    #[error("Flutter package config contains duplicate package name '{0}'")]
    DuplicatePackage(String),
    #[error("read current directory: {0}")]
    ReadCurrentDirectory(#[source] std::io::Error),
    #[error("create .dart_tool directory: {0}")]
    CreateDartTool(#[source] std::io::Error),
    #[error("serialize Flutter package config: {0}")]
    SerializeConfig(#[source] serde_json::Error),
    #[error("serialize Flutter package graph: {0}")]
    SerializeGraph(#[source] serde_json::Error),
    #[error(transparent)]
    Shared(#[from] MgError),
}

impl From<FlutterPackageConfigError> for MgError {
    fn from(error: FlutterPackageConfigError) -> Self {
        let message = error.to_string();
        match error {
            FlutterPackageConfigError::UnsafeDartTool
            | FlutterPackageConfigError::MissingGraphRoot
            | FlutterPackageConfigError::DuplicatePackage(_) => Self::Integrity(message),
            FlutterPackageConfigError::Shared(shared) => shared,
            FlutterPackageConfigError::InspectDartTool(_)
            | FlutterPackageConfigError::ReadCurrentDirectory(_)
            | FlutterPackageConfigError::CreateDartTool(_)
            | FlutterPackageConfigError::SerializeConfig(_)
            | FlutterPackageConfigError::SerializeGraph(_) => Self::Other(message),
        }
    }
}
