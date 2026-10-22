//! Local Ed25519 error taxonomy and compatibility mapping — Phân loại lỗi Ed25519 local và ánh xạ tương thích.
//! Preserve the established crate-level error variants and text — Giữ variant và text lỗi cấp crate hiện có.

#[derive(Debug, thiserror::Error)]
pub(in crate::ed25519_signer) enum Ed25519Error {
    #[error("Base64 decoding failed")]
    Base64Decode(#[from] base64::DecodeError),
    #[error("invalid signature length: {0}")]
    InvalidSignatureLength(usize),
    #[error("invalid public-key length: {0}")]
    InvalidPublicKeyLength(usize),
    #[error("seed rejected: {0}")]
    SeedRejected(String),
    #[error("PKCS#8 rejected: {0}")]
    Pkcs8Rejected(String),
    #[error("signature verification failed: {0}")]
    VerificationFailed(String),
}

impl From<Ed25519Error> for crate::CryptoError {
    fn from(error: Ed25519Error) -> Self {
        match error {
            Ed25519Error::Base64Decode(error) => Self::Base64Error(error),
            Ed25519Error::InvalidSignatureLength(length) => {
                Self::InvalidSignature(format!("expected 64 bytes, got {length}"))
            }
            Ed25519Error::InvalidPublicKeyLength(length) => {
                Self::InvalidPublicKey(format!("expected 32 bytes, got {length}"))
            }
            Ed25519Error::SeedRejected(message) => {
                Self::SigningFailed(format!("invalid seed: {message}"))
            }
            Ed25519Error::Pkcs8Rejected(message) => {
                Self::SigningFailed(format!("invalid pkcs8: {message}"))
            }
            Ed25519Error::VerificationFailed(message) => {
                Self::VerificationFailed(format!("signature invalid: {message}"))
            }
        }
    }
}
