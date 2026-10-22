//! message_digest owner wiring.
//! Wiring owner Quản lý domain thông điệp ký và digest BLAKE3.
mod message_digest;
pub use message_digest::{ATTESTATION_MESSAGE_DOMAIN, blake3_hex, canonical_message};
