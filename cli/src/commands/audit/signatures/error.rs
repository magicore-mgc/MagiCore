//! Typed coordinator failures mapped to stable signature-audit diagnostics.
//! Lỗi điều phối có kiểu, ánh xạ về diagnostic audit chữ ký ổn định.

#[derive(Debug, thiserror::Error)]
pub(super) enum SignatureAuditError {
    #[error("trusted provenance signature or transparency chain verification failed")]
    Invalid,
    #[error("registry returned no provenance entries or a package mismatch")]
    MissingOrMismatched,
    #[error("trusted publishing requires a valid registry URL without credentials or fragment")]
    RegistryUrlInvalid,
    #[error("trusted publishing requires HTTPS except for loopback registry URLs")]
    RequiresHttps,
    #[error("trusted signatures request failed with HTTP {0}")]
    RequestFailed(u16),
}

impl SignatureAuditError {
    /// Preserve the established CLI diagnostic strings and status mapping.
    /// Giữ chuỗi diagnostic CLI và ánh xạ trạng thái đã công bố.
    pub fn into_cli(self) -> anyhow::Error {
        match self {
            Self::Invalid => crate::error::trusted_signatures_invalid(),
            Self::MissingOrMismatched => crate::error::trusted_signatures_missing_or_mismatched(),
            Self::RegistryUrlInvalid => crate::error::trusted_registry_url_invalid(),
            Self::RequiresHttps => crate::error::trusted_registry_requires_https(),
            Self::RequestFailed(status) => crate::error::trusted_signatures_request_failed(status),
        }
    }
}
