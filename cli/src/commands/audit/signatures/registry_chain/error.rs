//! Typed registry-chain failures mapped to established CLI diagnostics.
//! Lỗi chuỗi registry có kiểu, ánh xạ về diagnostic CLI hiện hành.

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::audit::signatures) enum RegistryChainError {
    #[error("trusted provenance signature or transparency chain verification failed")]
    Invalid,
    #[cfg(test)]
    #[error("registry returned no provenance entries or a package mismatch")]
    MissingOrMismatched,
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
}

impl RegistryChainError {
    /// Preserve legacy messages while keeping serialization failures inspectable.
    /// Giữ message cũ nhưng vẫn giữ nguyên nhân lỗi serialization để truy vết.
    pub fn into_cli(self) -> anyhow::Error {
        match self {
            Self::Invalid => crate::error::trusted_signatures_invalid(),
            #[cfg(test)]
            Self::MissingOrMismatched => crate::error::trusted_signatures_missing_or_mismatched(),
            Self::Serialization(source) => anyhow::Error::new(source),
        }
    }
}
