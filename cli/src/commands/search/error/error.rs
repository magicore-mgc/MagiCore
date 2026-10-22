//! Preserve underlying registry, network and serialization errors.
//! Bảo toàn lỗi gốc từ registry, network và serialize.

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::search) enum SearchError {
    #[error("{0}")]
    Registry(#[from] anyhow::Error),
    #[error("{0}")]
    Request(#[source] reqwest::Error),
    #[error("{0}")]
    Response(#[source] reqwest::Error),
    #[error("{0}")]
    Serialization(#[from] serde_json::Error),
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
