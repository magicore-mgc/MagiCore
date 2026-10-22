//! Signature owner wiring — Kết nối owner chữ ký.

mod signature;
pub use signature::Ed25519Signature;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
