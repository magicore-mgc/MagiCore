//! Shared BLAKE3 digest decoder export.
//! Export decoder digest BLAKE3 dùng chung.

mod digest;

pub(super) use digest::blake3_digest_bytes;
