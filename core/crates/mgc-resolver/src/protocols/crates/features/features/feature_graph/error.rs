//! Typed failures while closing the Crates.io dependency graph.
//! Lỗi đóng graph dependency Crates.io có kiểu.

use super::super::super::super::index::IndexError;
use super::super::activation::FeatureActivationError;
use super::super::resolved_entry::ResolvedEntryError;
use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::protocols::crates) enum FeatureGraphError {
    #[error(transparent)]
    Index(#[from] IndexError),
    #[error(transparent)]
    Activation(#[from] FeatureActivationError),
    #[error(transparent)]
    ResolvedEntry(#[from] ResolvedEntryError),
    #[error(
        "crates.io package '{package}'@{version} requires dependency '{dependency}' from alternate registry '{registry}', which MagiCore cannot resolve yet"
    )]
    AlternateRegistry {
        package: String,
        version: String,
        dependency: String,
        registry: String,
    },
    #[error("crates.io index cache lost a requested package")]
    IndexCacheLost,
}

pub(in crate::protocols::crates::features::features) type FeatureGraphResult<T> =
    Result<T, FeatureGraphError>;

impl From<FeatureGraphError> for MgError {
    fn from(error: FeatureGraphError) -> Self {
        match error {
            FeatureGraphError::Index(error) => error.into(),
            FeatureGraphError::Activation(error) => error.into(),
            FeatureGraphError::ResolvedEntry(error) => error.into(),
            FeatureGraphError::AlternateRegistry {
                package,
                version,
                dependency,
                registry,
            } => Self::Other(format!(
                "crates.io package '{package}'@{version} requires dependency '{dependency}' from alternate registry '{registry}', which MagiCore cannot resolve yet"
            )),
            FeatureGraphError::IndexCacheLost => {
                Self::Other("crates.io index cache lost a requested package".to_string())
            }
        }
    }
}
