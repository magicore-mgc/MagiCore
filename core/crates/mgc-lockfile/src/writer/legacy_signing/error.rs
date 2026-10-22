//! Defines the local error for missing default signing identity.
//! Định nghĩa lỗi cục bộ khi thiếu identity ký mặc định.

#[derive(Debug, thiserror::Error)]
pub(super) enum LegacySigningError {
    #[error("no default key in keyring")]
    MissingDefaultKey,
}

impl From<LegacySigningError> for crate::LockfileError {
    fn from(error: LegacySigningError) -> Self {
        Self::VerificationFailed(error.to_string())
    }
}
