//! Legacy method compatibility wiring — Wiring tương thích method cũ.
//! Export the streaming compatibility method within BLAKE3 — Export method tương thích streaming trong BLAKE3.

mod mmap;
pub(in crate::blake3_signer) use mmap::hash_file_mmap;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
