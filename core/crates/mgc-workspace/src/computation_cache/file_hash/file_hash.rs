// Source-file digest — digest file source

use anyhow::Result;
use std::fs;
use std::path::Path;

/// Calculate the SHA-256 digest of a file.
/// Tính digest SHA-256 của một file.
pub fn hash_file(path: &Path) -> Result<String> {
    let bytes = fs::read(path)?;
    let hasher = blake3_or_sha256(&bytes);
    Ok(hasher)
}

fn blake3_or_sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}
