//! Typed Sigstore-validation errors with the prior CLI diagnostic boundary.
//! Lỗi kiểm tra Sigstore có kiểu, giữ ranh giới diagnostic CLI trước đây.

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::audit::signatures) enum SigstoreAttestationError {
    #[error("trusted provenance signature or transparency chain verification failed")]
    Invalid,
    #[error(transparent)]
    External(#[from] anyhow::Error),
}

impl SigstoreAttestationError {
    /// Keep validation failures generic and preserve previously exposed source errors.
    /// Giữ lỗi validation tổng quát và nguyên văn source từng được lộ trước đây.
    pub fn into_cli(self) -> anyhow::Error {
        match self {
            Self::Invalid => crate::error::trusted_signatures_invalid(),
            Self::External(source) => source,
        }
    }
}
