//! Base64 codec owner wiring — Wiring owner codec Base64.
//! Expose helpers only within the BLAKE3 module — Chỉ mở helper trong module BLAKE3.

mod base64;
pub(in crate::blake3_signer) use base64::{decode, encode};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
