//! Ed25519 error owner wiring — Wiring owner lỗi Ed25519.

mod error;
pub(super) use error::Ed25519Error;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
