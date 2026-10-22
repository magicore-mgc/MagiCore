//! Classifies lock mutation-policy failures before mapping to crate errors.
//! Phân loại lỗi policy mutation trước khi ánh xạ sang lỗi crate.

use std::io;

#[derive(Debug, thiserror::Error)]
pub(super) enum MutationPolicyError {
    #[error(
        "signature artifact '{path}' exists; verify it, explicitly remove the signature, perform the mutation, then sign again (automatic pair re-sign is not crash-atomic yet)"
    )]
    SignedArtifact { path: String },
    #[error("cannot inspect lock signature '{path}': {source}")]
    SignatureInspection {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("refusing to mutate non-regular lockfile path '{path}'")]
    NonRegularPath { path: String },
    #[error("lockfile '{path}' has no valid metadata table; refusing mutation")]
    MissingMetadata { path: String },
    #[error(
        "signed metadata remains in '{path}' but its signature sidecar is absent; explicitly verify and repair signing state before mutation"
    )]
    SignedMetadata { path: String },
    #[error(
        "legacy lockfile writer cannot mutate v4 lock '{path}'; use the schema-preserving v4 API"
    )]
    LegacyV4Path { path: String },
    #[error("cannot inspect lockfile '{path}': {source}")]
    LockInspection {
        path: String,
        #[source]
        source: io::Error,
    },
}

impl From<MutationPolicyError> for crate::LockfileError {
    fn from(error: MutationPolicyError) -> Self {
        let message = error.to_string();
        if matches!(
            &error,
            MutationPolicyError::SignedArtifact { .. } | MutationPolicyError::SignedMetadata { .. }
        ) {
            Self::SignedLockMutation(message)
        } else {
            Self::WriteFailed(message)
        }
    }
}
