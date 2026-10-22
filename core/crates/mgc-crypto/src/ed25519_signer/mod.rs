//! Ed25519 signing and verification API — API ký và xác minh Ed25519.
//! Preserve the existing public module and crate-root paths — Giữ module public và crate-root path hiện có.

mod ed25519_signer;
mod encoding;
mod error;
mod public_key;
mod signature;
mod verification;

pub use ed25519_signer::Ed25519Signer;
pub use public_key::Ed25519PublicKey;
pub use signature::Ed25519Signature;
pub use verification::{verify_signature, verify_string_signature};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
