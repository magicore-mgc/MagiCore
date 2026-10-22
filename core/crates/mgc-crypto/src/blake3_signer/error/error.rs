//! Local parse error taxonomy mapped at the stable crypto API boundary.
//! Phân loại lỗi parse cục bộ, ánh xạ tại ranh giới API crypto ổn định.

#[derive(Debug, thiserror::Error)]
pub(in crate::blake3_signer) enum EncodingError {
    #[error("{0}")]
    InvalidHex(String),
    #[error("{0}")]
    InvalidBase64(String),
    #[error("expected 32 bytes, got {0}")]
    InvalidDigestLength(usize),
}
