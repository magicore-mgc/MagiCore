//! Secret-free credential failures.
//! Lỗi credential không chứa bí mật.

#[derive(Debug)]
pub(crate) enum CredentialError {
    Parameters,
    Hashing,
    WorkerUnavailable,
}

impl std::fmt::Display for CredentialError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Parameters => "credential KDF parameters are invalid",
            Self::Hashing => "credential password hashing failed",
            Self::WorkerUnavailable => "credential worker is unavailable",
        })
    }
}

impl std::error::Error for CredentialError {}
