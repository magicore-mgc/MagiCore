//! Hash operation owner wiring — Wiring owner thao tác hash.
//! Export the stable hasher API and register local tests — Export API hasher ổn định và đăng ký test local.

mod hash;
pub use hash::{Blake3Hasher, Blake3StreamingHasher};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
