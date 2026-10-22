//! Write Cargo's checksum only after verifying the archive and extracted source.
//! Tạo checksum Cargo sau khi xác minh archive và nội dung source đã giải nén.

use crate::install::cargo_compat::{CargoChecksum, atomic_write};
use mgc_types::{MgError, MgResult};
use std::path::Path;

use super::super::canonical_file_map::source_files_match_archive;
use super::super::tree_hash::hash_source_files;
use super::super::validation::validate_sha256;

/// Write Cargo's directory-source checksum after MGC has verified and extracted a crate.
/// Ghi checksum cho Cargo directory source sau khi MGC đã xác minh và giải nén crate.
pub(crate) fn write_source_checksum(
    package_root: &Path,
    package_sha256: &str,
    archive_bytes: &[u8],
) -> MgResult<()> {
    use sha2::{Digest, Sha256};

    validate_sha256(package_sha256)?;
    let archive_sha256 = hex::encode(Sha256::digest(archive_bytes));
    if !archive_sha256.eq_ignore_ascii_case(package_sha256) {
        return Err(MgError::Integrity(
            "Cargo crate archive digest changed before source projection".to_string(),
        ));
    }
    let expected_root = package_root
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| MgError::Other("Cargo source package path is invalid".to_string()))?;
    let files =
        mgc_resolver::protocols::archive::cargo_source_file_hashes(archive_bytes, expected_root)
            .map_err(|error| MgError::Integrity(error.to_string()))?;
    let actual_files = hash_source_files(package_root)?;
    if !source_files_match_archive(package_root, &files, &actual_files)? {
        return Err(MgError::Integrity(
            "extracted Cargo source does not match its verified archive on this filesystem"
                .to_string(),
        ));
    }
    let checksum = CargoChecksum {
        files,
        package: &archive_sha256,
    };
    let bytes = serde_json::to_vec(&checksum)
        .map_err(|error| MgError::Other(format!("serialize Cargo source checksum: {error}")))?;
    atomic_write(package_root, Path::new(".cargo-checksum.json"), &bytes)
}
