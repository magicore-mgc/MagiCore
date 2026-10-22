//! Verify raw patch bytes with SHA-256.
//! Xác minh byte patch bằng SHA-256.

use super::error::IntegrityError;
use anyhow::Result;
use std::{fs, path::Path};

/// Verify patch integrity (SHA256 of patch file content).
/// Xác minh tính toàn vẹn patch bằng SHA-256 của nội dung file.
pub fn verify_patch_integrity(patch_path: &Path, expected_sha256: &str) -> Result<bool> {
    use sha2::{Digest, Sha256};
    let content = fs::read(patch_path).map_err(IntegrityError::from)?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let actual = hex::encode(hasher.finalize());
    // Accept the SRI form (`sha256-<hex>`, what `mgc patch add` records)
    // as well as bare hex — comparing raw against prefixed can never
    // match, which made every added patch fail verification.
    // (Chấp nhận cả dạng SRI lẫn hex trần.)
    let expected = expected_sha256
        .strip_prefix("sha256-")
        .unwrap_or(expected_sha256);
    Ok(actual == expected)
}

#[cfg(test)]
#[path = "test/integrity.rs"]
mod tests;
