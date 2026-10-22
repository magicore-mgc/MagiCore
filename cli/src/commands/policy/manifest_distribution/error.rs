//! Typed URL, OCI and cache refresh errors.
//! Lỗi typed cho refresh URL, OCI và cache.

use crate::commands::policy::manifest_cache::PolicyManifestCacheError;

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::policy) enum PolicyManifestDistributionError {
    #[error("policy manifest URL invalid: {detail}")]
    InvalidUrl {
        detail: String,
        #[source]
        source: url::ParseError,
    },
    #[error("policy manifest URL must use HTTPS unless it targets loopback")]
    InsecureUrl,
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error("policy OCI reference invalid: '{0}'")]
    InvalidOciReference(String),
    #[error(transparent)]
    OciClient(#[from] anyhow::Error),
    #[error("OCI artifact is not a policy manifest (config media type '{0}')")]
    OciMediaType(String),
    #[error("policy OCI artifact must not carry layers")]
    OciLayers,
    #[error(transparent)]
    Cache(#[from] PolicyManifestCacheError),
}

pub(in crate::commands::policy) type PolicyManifestDistributionResult<T> =
    std::result::Result<T, PolicyManifestDistributionError>;
