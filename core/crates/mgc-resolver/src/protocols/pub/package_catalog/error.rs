//! Typed package-catalog failures mapped to the shared protocol error contract.
//! Lỗi catalog có kiểu riêng và được ánh xạ về contract lỗi protocol dùng chung.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(super) enum CatalogError {
    #[error("invalid package name: {0}")]
    InvalidPackageName(String),
    #[error("no pub.dev version of '{name}' satisfies all constraints: {constraints}")]
    NoMatchingVersion { name: String, constraints: String },
    #[error("package {name} {version} has no archive_url")]
    MissingArchiveUrl { name: String, version: String },
    #[error(
        "pub.dev package {name} {version} has no valid archive_sha256; refusing to resolve an unverifiable artifact"
    )]
    MissingArchiveDigest { name: String, version: String },
    #[error("pub.dev version {name} {version} is missing pubspec metadata")]
    MissingPubspec { name: String, version: String },
    #[error(
        "dependency '{name}' uses {source_kind}; the native pub.dev resolver cannot claim or substitute it"
    )]
    UnsupportedDependencySource { name: String, source_kind: String },
    #[error("dependency '{name}' has an empty version constraint")]
    EmptyConstraint { name: String },
}

impl CatalogError {
    pub(super) fn into_mg_error(self) -> MgError {
        match self {
            Self::InvalidPackageName(name) => MgError::InvalidPackageName(name),
            Self::NoMatchingVersion { name, constraints } => MgError::DependencyConflict(format!(
                "no pub.dev version of '{name}' satisfies all constraints: {constraints}"
            )),
            Self::MissingArchiveUrl { name, version } => {
                MgError::Other(format!("package {name} {version} has no archive_url"))
            }
            Self::MissingArchiveDigest { name, version } => MgError::Integrity(format!(
                "pub.dev package {name} {version} has no valid archive_sha256; refusing to resolve an unverifiable artifact"
            )),
            Self::MissingPubspec { name, version } => MgError::Other(format!(
                "pub.dev version {name} {version} is missing pubspec metadata"
            )),
            Self::UnsupportedDependencySource { name, source_kind } => MgError::Unsupported {
                core: "app",
                capability: "pub.dev dependency source",
                guidance: format!(
                    "dependency '{name}' uses {source_kind}; the native pub.dev resolver cannot claim or substitute it"
                ),
            },
            Self::EmptyConstraint { name } => MgError::Unsupported {
                core: "app",
                capability: "pub.dev dependency constraint",
                guidance: format!("dependency '{name}' has an empty version constraint"),
            },
        }
    }
}
