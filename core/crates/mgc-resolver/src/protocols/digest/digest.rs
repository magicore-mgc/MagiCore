//! Shared registry artifact digest and CAS-reference helpers.
//! Helper digest artifact và CAS reference dùng chung cho registry.

use sha2::{Digest, Sha256};

/// sha256 of bytes as lowercase hex — SỞ HỮU chung cho mọi engine.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

/// Format a blake3 hex digest as the mgc-store CAS ref (`files/blake3/xx/hash`).
/// Định dạng digest blake3 hex thành CAS ref của mgc-store.
pub fn store_ref_for_blake3(hex_digest: &str) -> String {
    let prefix = &hex_digest[..2.min(hex_digest.len())];
    format!("files/blake3/{prefix}/{hex_digest}")
}

#[cfg(test)]
#[path = "test/digest.rs"]
mod tests;
