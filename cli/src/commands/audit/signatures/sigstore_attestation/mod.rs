//! Public Sigstore verification and in-toto statement binding.
//! Xác minh Sigstore công khai và ràng buộc statement in-toto.

mod error;
pub(super) mod sigstore_attestation;

pub(super) use error::SigstoreAttestationError;
pub(super) use sigstore_attestation::{production_verifier, verify_public_attestation};

#[cfg(test)]
#[path = "test/sigstore_attestation.rs"]
mod tests;

#[cfg(test)]
pub(super) use sigstore_attestation::{
    DEFAULT_WORKFLOW_IDENTITY_PREFIX, workflow_identity_allowed,
};
