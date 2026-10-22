//! Typed failures for Cargo manifest parsing and projection.
//! Lỗi có kiểu cho parse và projection manifest Cargo.

use mgc_types::MgError;

/// Retain precise local failure categories and their underlying parse sources.
/// Giữ phân loại lỗi cục bộ và source gốc của lỗi parse.
#[derive(Debug, thiserror::Error)]
pub(in crate::install::cargo_compat::manifest) enum CargoManifestError {
    #[error("parse Cargo.toml for Cargo projection: {0}")]
    TomlParse(#[source] toml::de::Error),
    #[error("invalid Cargo.toml package.version: {0}")]
    InvalidVersion(#[source] MgError),
    #[error(transparent)]
    ProjectFile(#[from] MgError),
    #[error("Cargo projection requires a package manifest")]
    PackageManifestRequired,
    #[error("Cargo projection does not support workspace-inherited package metadata yet")]
    WorkspaceInheritedMetadataUnsupported,
    #[error("Cargo projection does not support workspaces yet")]
    WorkspaceUnsupported,
    #[error("Cargo projection does not support Cargo.toml [{section}] yet")]
    UnsupportedSection { section: &'static str },
    #[error("Cargo projection does not support custom root features yet")]
    CustomRootFeaturesUnsupported,
    #[error("{label} is required for Cargo projection")]
    RequiredString { label: String },
    #[error("Cargo.toml [dependencies] must be a table")]
    DependenciesTableRequired,
    #[error("mgc.lock root '{root_id}' is missing from the Rust package graph")]
    LockRootMissing { root_id: String },
    #[error("mgc.lock root '{name}' is absent from Cargo.toml [dependencies]")]
    RootAbsentFromManifest { name: String },
    #[error("mgc.lock direct roots do not match Cargo.toml [dependencies]; run `mgc install`")]
    DirectRootsMismatch,
    #[error(
        "Cargo projection does not support path/git/workspace/alias/registry dependency '{name}'"
    )]
    UnsupportedDependencyOrigin { name: String },
    #[error(
        "Cargo projection does not support optional dependency '{name}' until root feature activation is modeled"
    )]
    OptionalDependencyUnsupported { name: String },
    #[error("Cargo projection requires a version string for dependency '{name}'")]
    DependencyVersionRequired { name: String },
    #[error("Cargo projection requires dependency '{name}' features to be non-empty strings")]
    DependencyFeaturesInvalid { name: String },
    #[error("Cargo projection requires dependency '{name}' default-features to be a boolean")]
    DependencyDefaultFeaturesInvalid { name: String },
    #[error("Cargo projection does not support an unknown dependency option for '{name}'")]
    UnknownDependencyOption { name: String },
    #[error("Cargo projection requires a version string or version table for dependency '{name}'")]
    DependencySpecInvalid { name: String },
    #[error("Cargo dependency '{name}' is absent from mgc.lock; run `mgc install`")]
    DependencyMissingFromLock { name: String },
}

impl From<CargoManifestError> for MgError {
    fn from(error: CargoManifestError) -> Self {
        match error {
            CargoManifestError::ProjectFile(source) => source,
            CargoManifestError::TomlParse(source) => {
                Self::Anyhow(anyhow::Error::new(CargoManifestError::TomlParse(source)))
            }
            CargoManifestError::InvalidVersion(source) => Self::Anyhow(anyhow::Error::new(
                CargoManifestError::InvalidVersion(source),
            )),
            validation_error => Self::Other(validation_error.to_string()),
        }
    }
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
