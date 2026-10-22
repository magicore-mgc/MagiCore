//! Typed cache/network failures for tarball retrieval.
//! Lỗi cache/network có kiểu cho luồng lấy tarball.

use mgc_types::MgError;

use super::super::StagingError;

#[derive(Debug, thiserror::Error)]
pub(in crate::install::fetch) enum FetchError {
    #[error("{0}")]
    LocalCacheRead(#[source] anyhow::Error),
    #[error("{0}")]
    SharedCacheRead(#[source] anyhow::Error),
    #[error("{0}")]
    CacheWrite(#[source] anyhow::Error),
    #[error("no registry URL provided for '{package}' and no cache hit")]
    MissingRegistry { package: String },
    #[error("download semaphore closed: {0}")]
    SemaphoreClosed(#[source] tokio::sync::AcquireError),
    #[error("download failed for '{package}': {source}")]
    RegistryDownload {
        package: String,
        #[source]
        source: anyhow::Error,
    },
    #[error(transparent)]
    Upstream(#[from] MgError),
    #[error(transparent)]
    Staging(#[from] StagingError),
}

impl From<FetchError> for MgError {
    fn from(error: FetchError) -> Self {
        match error {
            FetchError::LocalCacheRead(source)
            | FetchError::SharedCacheRead(source)
            | FetchError::CacheWrite(source) => Self::Store(source.to_string()),
            FetchError::MissingRegistry { package } => Self::Network(format!(
                "no registry URL provided for '{package}' and no cache hit"
            )),
            FetchError::SemaphoreClosed(source) => {
                Self::Other(format!("download semaphore closed: {source}"))
            }
            FetchError::RegistryDownload { package, source } => {
                Self::Network(format!("download failed for '{package}': {source}"))
            }
            FetchError::Upstream(error) => error,
            FetchError::Staging(error) => Self::Store(error.to_string()),
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
