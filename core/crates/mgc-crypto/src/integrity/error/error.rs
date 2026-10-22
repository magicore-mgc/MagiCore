//! Local SRI failure taxonomy and public error mapping — Phân loại lỗi SRI local và map lỗi public.
//! Retain wrapped error sources and exact legacy messages — Giữ source lỗi được bọc và message legacy chính xác.

#[derive(Debug, thiserror::Error)]
pub(in crate::integrity) enum IntegrityError {
    #[error("invalid SRI format: {0}")]
    InvalidSriFormat(String),
    #[error("algorithm not supported: {0}")]
    UnsupportedAlgorithm(String),
    #[error("unknown algorithm: {0}")]
    UnknownAlgorithm(String),
    #[error("BLAKE3 hash mismatch")]
    HashMismatch { expected: String, actual: String },
    #[error(transparent)]
    Crypto(#[from] crate::CryptoError),
    #[error(transparent)]
    FileIo(#[from] std::io::Error),
}

impl From<IntegrityError> for crate::CryptoError {
    fn from(error: IntegrityError) -> Self {
        match error {
            IntegrityError::InvalidSriFormat(value) => {
                Self::Blake3Failed(format!("invalid SRI format: {value}"))
            }
            IntegrityError::UnsupportedAlgorithm(algorithm) => {
                Self::Blake3Failed(format!("algorithm not supported: {algorithm} (use blake3)"))
            }
            IntegrityError::UnknownAlgorithm(algorithm) => {
                Self::Blake3Failed(format!("unknown algorithm: {algorithm}"))
            }
            IntegrityError::HashMismatch { expected, actual } => Self::VerificationFailed(format!(
                "BLAKE3 hash mismatch: expected {expected}, got {actual}"
            )),
            IntegrityError::Crypto(error) => error,
            IntegrityError::FileIo(error) => Self::IoError(error),
        }
    }
}
