//! Typed verification-flow failures mapped to stable security error classes.
//! Lỗi luồng verify có kiểu, ánh xạ về nhóm lỗi bảo mật ổn định.

/// Security failures created by the sidecar verification coordinator.
/// Lỗi bảo mật do coordinator xác minh sidecar tạo ra.
#[derive(Debug, thiserror::Error)]
pub(super) enum VerificationFlowError {
    #[error("hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },
    #[error("no signer info in lockfile")]
    MissingSigner,
    #[error("signer key ID does not match the public-key fingerprint")]
    KeyIdMismatch,
    #[error("signature verification failed: {0}")]
    SignatureCheck(String),
}

impl From<VerificationFlowError> for crate::LockfileError {
    fn from(error: VerificationFlowError) -> Self {
        let message = error.to_string();
        match error {
            VerificationFlowError::HashMismatch { .. } => Self::TamperedLockfile(message),
            _ => Self::VerificationFailed(message),
        }
    }
}
