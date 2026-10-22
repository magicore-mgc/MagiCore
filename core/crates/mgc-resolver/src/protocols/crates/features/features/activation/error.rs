//! Typed invalid Cargo feature declarations.
//! Khai báo Cargo feature không hợp lệ có kiểu.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::protocols::crates) enum FeatureActivationError {
    #[error(
        "crates.io package '{package}'@{version} does not define requested feature '{feature}'"
    )]
    UndefinedFeature {
        package: String,
        version: String,
        feature: String,
    },
    #[error(
        "crates.io feature '{feature}' references missing dependency '{alias}' in '{package}'@{version}"
    )]
    MissingDependency {
        package: String,
        version: String,
        feature: String,
        alias: String,
    },
    #[error(
        "crates.io feature '{feature}' has invalid dependency feature '{value}' in '{package}'@{version}"
    )]
    InvalidDependencyFeature {
        package: String,
        version: String,
        feature: String,
        value: String,
    },
    #[error(
        "crates.io feature '{feature}' references unknown feature or dependency '{value}' in '{package}'@{version}"
    )]
    UnknownFeatureReference {
        package: String,
        version: String,
        feature: String,
        value: String,
    },
}

pub(in crate::protocols::crates::features::features) type FeatureActivationResult<T> =
    Result<T, FeatureActivationError>;

impl From<FeatureActivationError> for MgError {
    fn from(error: FeatureActivationError) -> Self {
        Self::Other(error.to_string())
    }
}
