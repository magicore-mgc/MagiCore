//! SRI-format SHA-256 hashing for tarball bytes.
//! Hash SHA-256 định dạng SRI cho byte tarball.

use base64::Engine;

/// Compute SHA-256 hash of bytes and return as base64 string (SRI format).
/// Hàm helper cho integrity verification — tính hash của tarball bytes.
pub fn compute_sha256_b64_str(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    base64::engine::general_purpose::STANDARD.encode(hasher.finalize())
}
