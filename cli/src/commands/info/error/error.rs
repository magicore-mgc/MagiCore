//! Typed package-info failures.
//! Các lỗi có kiểu của lệnh thông tin package.

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::info) enum InfoError {
    #[cfg(not(feature = "web"))]
    #[error(
        "package info currently requires the web core registry adapter, which is not included in this build"
    )]
    NoWebAdapter,
    #[cfg(feature = "web")]
    #[error("Could not fetch package info from registry: {source}")]
    RegistryFetchFailed {
        #[source]
        source: anyhow::Error,
    },
    #[cfg(feature = "web")]
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
