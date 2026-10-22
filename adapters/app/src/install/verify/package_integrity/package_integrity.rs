//! SHA-256 checks for materialized App package files.
//! Kiểm tra SHA-256 cho file package App đã materialize.
use mgc_types::{MgError, MgResult};
use sha2::{Digest, Sha256};
use std::path::Path;

/// Verify package file integrity with SHA-256.
/// Kiểm tra integrity file package với SHA-256.
pub fn verify_package_file(file_path: &Path, expected_hash: Option<&str>) -> MgResult<()> {
    if expected_hash.is_none() {
        // No hash available: warn but allow (permissive mode)
        eprintln!(
            "WARNING: No integrity hash for package '{}'",
            file_path.display()
        );
        return Ok(());
    }

    let Some(expected) = expected_hash else {
        // Guarded above, but re-check instead of unwrapping: a refactor
        // that moves the guard must fail closed, not panic.
        return Err(MgError::Other(format!(
            "cannot verify package '{}': expected hash vanished after the availability check",
            file_path.display()
        )));
    };
    let actual = compute_sha256_file(file_path)?;

    if actual.eq_ignore_ascii_case(expected) {
        Ok(())
    } else {
        Err(MgError::Other(format!(
            "integrity mismatch for '{}': expected {}, got {}",
            file_path.display(),
            expected,
            actual
        )))
    }
}

/// Compute SHA-256 hash of a file.
/// Tính SHA-256 hash của file.
fn compute_sha256_file(path: &Path) -> MgResult<String> {
    let bytes =
        std::fs::read(path).map_err(|e| MgError::Other(format!("failed to read file: {}", e)))?;

    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(hex::encode(hasher.finalize()))
}

/// Verify Flutter package archive SHA-256 from pubspec.lock.
/// Kiểm tra SHA-256 archive Flutter package từ pubspec.lock.
pub fn verify_flutter_archive_sha256(archive_path: &Path, expected_sha256: &str) -> MgResult<()> {
    verify_package_file(archive_path, Some(expected_sha256))
}
