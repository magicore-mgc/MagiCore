//! attestation owner wiring.
//! Wiring owner Điều phối validation, idempotency, chọn key và tạo attestation.
mod attestation;
pub use attestation::{attest, keyring_public_key, pin_key_in_manifest};
