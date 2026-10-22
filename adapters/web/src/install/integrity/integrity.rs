//! Tarball integrity verification (SRI checksums).
//! Xác minh integrity tarball (checksum SRI).

use base64::Engine;
use mgc_types::adapter::ResolvedPackage;
use mgc_types::{MgError, MgResult};

use crate::lockfile::{compute_sha512_b64, compute_tarball_integrity, strict_integrity_enforced};

/// Compute SHA-256 hash of bytes and return as base64 string (SRI format).
/// Hàm helper cho integrity verification — tính hash của tarball bytes.
pub fn compute_sha256_b64_str(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    base64::engine::general_purpose::STANDARD.encode(hasher.finalize())
}

/// Verify tarball integrity against package SRI field.
/// Wrapper cho verify_sri_integrity — entry point chính.
pub fn verify_tarball_integrity(pkg: &ResolvedPackage, bytes: &[u8]) -> MgResult<()> {
    verify_sri_integrity(pkg, bytes)
}

/// Prepare package for cache by computing integrity if missing, then verify.
/// Chuẩn bị package trước khi cache — compute integrity nếu thiếu, rồi verify.
pub fn prepare_verified_tarball_for_cache(pkg: &mut ResolvedPackage, bytes: &[u8]) -> MgResult<()> {
    // Check supplied evidence before deriving the cache identity.
    // Strict mode phải từ chối SRI thiếu trước khi tự tính hash cho cache.
    verify_tarball_integrity(pkg, bytes)?;
    if pkg.integrity.is_empty() {
        pkg.integrity = compute_tarball_integrity(bytes);
    }
    Ok(())
}

/// Verify SRI (Subresource Integrity) checksums for tarball bytes.
/// Kiểm tra integrity field của package khớp với bytes thật.
/// Hỗ trợ sha256/sha512 (strong), cảnh báo sha1/md5 (weak).
pub fn verify_sri_integrity(pkg: &ResolvedPackage, bytes: &[u8]) -> MgResult<()> {
    verify_sri_hashes(pkg, |algorithm| match algorithm {
        "sha256" => compute_sha256_b64_str(bytes),
        "sha512" => compute_sha512_b64(bytes),
        _ => unreachable!("only supported strong algorithms reach the hash provider"),
    })
}

// Hash the owned staging file with bounded memory before publishing it.
// File streaming phải qua cùng policy SRI, không tự cấp bằng chứng khi SRI thiếu.
pub fn verify_tarball_file_integrity(
    pkg: &ResolvedPackage,
    path: &std::path::Path,
) -> MgResult<()> {
    let file = std::fs::File::open(path)
        .map_err(|error| MgError::Store(format!("cannot open staged tarball: {error}")))?;
    verify_tarball_reader_integrity(pkg, file)
}

pub fn verify_tarball_reader_integrity<R: std::io::Read>(
    pkg: &ResolvedPackage,
    mut reader: R,
) -> MgResult<()> {
    use sha2::{Digest, Sha256, Sha512};
    const FILE_HASH_BUFFER_BYTES: usize = 64 * 1024;
    if pkg.integrity.is_empty() {
        return verify_sri_hashes(pkg, |_| String::new());
    }
    let mut sha256 = Sha256::new();
    let mut sha512 = Sha512::new();
    let mut buffer = [0u8; FILE_HASH_BUFFER_BYTES];
    loop {
        let read = reader.read(&mut buffer).map_err(|error| {
            MgError::Store(format!(
                "cannot read tarball for integrity verification: {error}"
            ))
        })?;
        if read == 0 {
            break;
        }
        sha256.update(&buffer[..read]);
        sha512.update(&buffer[..read]);
    }
    let sha256 = base64::engine::general_purpose::STANDARD.encode(sha256.finalize());
    let sha512 = base64::engine::general_purpose::STANDARD.encode(sha512.finalize());
    verify_sri_hashes(pkg, |algorithm| match algorithm {
        "sha256" => sha256.clone(),
        "sha512" => sha512.clone(),
        _ => unreachable!("only supported strong algorithms reach the hash provider"),
    })
}

// Share policy decisions between byte buffers and file-backed downloads.
// Chỉ nguồn hash khác nhau; chính sách strict và thuật toán giữ chung một chỗ.
fn verify_sri_hashes(pkg: &ResolvedPackage, hash: impl Fn(&str) -> String) -> MgResult<()> {
    // Empty integrity: strict mode fail, permissive mode warn
    // Integrity trống: strict mode fail, permissive mode cho qua
    if pkg.integrity.is_empty() {
        if strict_integrity_enforced() {
            return Err(MgError::Other(format!(
                "strict integrity: '{}' has no SRI integrity field",
                pkg.id.name_str()
            )));
        }
        return Ok(());
    }

    let mut has_weak_algorithm = false;
    let mut has_strong_algorithm = false;

    // Parse SRI format: "sha512-base64 sha256-base64" (space separated)
    // Parse format SRI: "sha512-base64 sha256-base64" (cách nhau bởi space)
    for entry in pkg.integrity.split_whitespace() {
        let Some((algorithm, expected)) = entry.split_once('-') else {
            continue;
        };

        // Reject weak algorithms in strict mode, warn in permissive
        // Reject thuật toán yếu ở strict mode, cảnh báo ở permissive
        if matches!(algorithm, "sha1" | "md5") {
            has_weak_algorithm = true;
            if strict_integrity_enforced() {
                return Err(MgError::Other(format!(
                    "strict integrity: '{}' uses weak hash algorithm '{}' (only sha256/sha512 allowed)",
                    pkg.id.name_str(),
                    algorithm
                )));
            }
            eprintln!(
                "WARNING: Package '{}' uses weak hash algorithm '{}', consider updating",
                pkg.id.name_str(),
                algorithm
            );
            continue;
        }

        // Compute actual hash for strong algorithms
        // Tính hash thật cho thuật toán mạnh
        let actual = match algorithm {
            "sha256" => {
                has_strong_algorithm = true;
                hash("sha256")
            }
            "sha512" => {
                has_strong_algorithm = true;
                hash("sha512")
            }
            _ => {
                eprintln!(
                    "WARNING: Package '{}' uses unknown hash algorithm '{}'",
                    pkg.id.name_str(),
                    algorithm
                );
                continue;
            }
        };

        // Match found: integrity OK
        // Tìm thấy match: integrity OK
        if actual == expected {
            return Ok(());
        }
    }

    // Only weak algorithms present: fail
    // Chỉ có thuật toán yếu: fail
    if has_weak_algorithm && !has_strong_algorithm {
        return Err(MgError::Other(format!(
            "integrity check failed for '{}': only weak algorithms present (sha1/md5)",
            pkg.id.name_str()
        )));
    }

    // None of the hashes matched
    // Không hash nào match
    Err(MgError::Other(format!(
        "integrity mismatch for '{}': none of the SRI entries matched",
        pkg.id.name_str()
    )))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
