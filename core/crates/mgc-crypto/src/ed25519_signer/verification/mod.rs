//! Signature verification owner wiring — Wiring owner xác minh chữ ký.

mod verification;
pub use verification::{verify_signature, verify_string_signature};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
