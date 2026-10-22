//! Shared public crypto error variants — Các variant lỗi crypto public dùng chung.
//! Preserve display text and wrapped source chains — Giữ display text và source chain được bọc.

/// Errors returned by `mgc-crypto` operations — Lỗi trả về từ thao tác `mgc-crypto`.
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Blake3 hash failed: {0}")]
    Blake3Failed(String),

    #[error("Ed25519 signing failed: {0}")]
    SigningFailed(String),

    #[error("Ed25519 verification failed: {0}")]
    VerificationFailed(String),

    #[error("Keyring operation failed: {0}")]
    KeyringFailed(String),

    #[error("Invalid signature format: {0}")]
    InvalidSignature(String),

    #[error("Invalid public key format: {0}")]
    InvalidPublicKey(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),
}
