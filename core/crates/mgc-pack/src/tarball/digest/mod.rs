//! Streamed archive digest exports — export digest archive dạng stream.

mod digest;

pub(super) use digest::digest_archive;

#[cfg(test)]
#[path = "test/digest.rs"]
mod tests;
