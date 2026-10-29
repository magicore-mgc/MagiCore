//! Typed Packagist failures mapped to the shared protocol error contract.
//! Lỗi Packagist có kiểu riêng, ánh xạ về contract lỗi protocol dùng chung.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::protocols::composer) enum ComposerError {
    #[error("invalid Composer package name '{0}' (expected vendor/package)")]
    InvalidPackageName(String),
    #[error("no Packagist version of '{name}' satisfies all constraints: {constraints}")]
    NoMatchingVersion { name: String, constraints: String },
    #[error("Packagist metadata for '{name}' does not list that package")]
    MissingPackage { name: String },
    #[error("Packagist package {name} {version} has no dist.url")]
    MissingDistUrl { name: String, version: String },
    #[error(
        "Packagist package {name} {version} publishes no dist shasum or dist reference; refusing to resolve an unverifiable artifact"
    )]
    MissingDistIntegrity { name: String, version: String },
    #[error(
        "Packagist package {name} {version} publishes a malformed dist shasum '{shasum}' (expected sha1 hex)"
    )]
    MalformedDistShasum {
        name: String,
        version: String,
        shasum: String,
    },
    #[error("dependency '{name}' has an empty version constraint")]
    EmptyConstraint { name: String },
}

impl ComposerError {
    pub(in crate::protocols::composer) fn into_mg_error(self) -> MgError {
        match self {
            Self::InvalidPackageName(name) => MgError::InvalidPackageName(name),
            Self::NoMatchingVersion { name, constraints } => MgError::DependencyConflict(format!(
                "no Packagist version of '{name}' satisfies all constraints: {constraints}"
            )),
            Self::MissingPackage { name } => {
                MgError::Other(format!("Packagist metadata for '{name}' lists no package"))
            }
            Self::MissingDistUrl { name, version } => {
                MgError::Other(format!("Packagist package {name} {version} has no dist.url"))
            }
            Self::MissingDistIntegrity { name, version } => MgError::Integrity(format!(
                "Packagist package {name} {version} publishes no dist shasum or dist reference; refusing to resolve an unverifiable artifact"
            )),
            Self::MalformedDistShasum {
                name,
                version,
                shasum,
            } => MgError::Integrity(format!(
                "Packagist package {name} {version} publishes a malformed dist shasum '{shasum}' (expected sha1 hex)"
            )),
            Self::EmptyConstraint { name } => MgError::Unsupported {
                core: "web",
                capability: "Composer dependency constraint",
                guidance: format!("dependency '{name}' has an empty version constraint"),
            },
        }
    }
}
