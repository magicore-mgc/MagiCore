//! Hex codec owner wiring — Wiring owner codec hex.
//! Expose helpers only within the BLAKE3 module — Chỉ mở helper trong module BLAKE3.

mod hex;
pub(in crate::blake3_signer) use hex::{decode, encode};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
