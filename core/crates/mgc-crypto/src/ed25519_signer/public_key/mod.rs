//! Public-key owner wiring — Kết nối owner khóa công khai.

mod public_key;
pub use public_key::Ed25519PublicKey;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
