//! Streaming file-hash wiring — Wiring owner hash file streaming.
//! Export only to sibling operations inside BLAKE3 — Chỉ export cho sibling trong BLAKE3.

mod stream;
pub(in crate::blake3_signer) use stream::hash_file;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
