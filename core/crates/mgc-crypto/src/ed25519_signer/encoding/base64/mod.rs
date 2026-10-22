//! Base64 helper wiring — Nối helper Base64.

mod base64;
pub(in crate::ed25519_signer) use base64::{decode, encode};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
