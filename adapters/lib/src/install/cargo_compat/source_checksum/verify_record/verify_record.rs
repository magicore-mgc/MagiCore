//! Read and verify Cargo's checksum against the locked archive and source tree.
//! Đọc và xác minh checksum Cargo với archive đã khóa và source tree trên đĩa.

use mgc_types::{MgError, MgResult};
use std::collections::BTreeMap;
use std::path::Path;

use super::super::canonical_file_map::source_files_match_archive;
use super::super::tree_hash::hash_source_files;
use super::super::validation::validate_sha256;

pub(crate) fn verify_source_checksum(
    package_root: &Path,
    expected_package_sha256: &str,
    archive_bytes: &[u8],
) -> MgResult<()> {
    validate_sha256(expected_package_sha256)?;
    use sha2::{Digest, Sha256};
    let archive_sha256 = hex::encode(Sha256::digest(archive_bytes));
    if !archive_sha256.eq_ignore_ascii_case(expected_package_sha256) {
        return Err(MgError::Integrity(
            "Cargo crate archive digest differs from the MGC lock".to_string(),
        ));
    }
    let expected_root = package_root
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| MgError::Other("Cargo source package path is invalid".to_string()))?;
    let expected_files =
        mgc_resolver::protocols::archive::cargo_source_file_hashes(archive_bytes, expected_root)
            .map_err(|error| MgError::Integrity(error.to_string()))?;
    let checksum_path = package_root.join(".cargo-checksum.json");
    // A missing record is recoverable: the caller holds the archive, which is
    // the source of truth, so rewrite the record instead of failing the whole
    // install. A record that exists but disagrees is still a hard error.
    // (Thiếu record là có thể phục hồi: caller có archive là nguồn chuẩn, nên
    // ghi lại record thay vì làm hỏng install. Record có mà lệch vẫn là lỗi.)
    if !checksum_path.is_file() {
        super::super::write_record::write_source_checksum(
            package_root,
            expected_package_sha256,
            archive_bytes,
        )?;
        return Ok(());
    }
    let checksum_text =
        mgc_adapter_base::project_file::read_regular_text(&checksum_path, ".cargo-checksum.json")?;
    let checksum: serde_json::Value = serde_json::from_str(&checksum_text)
        .map_err(|error| MgError::Other(format!("invalid Cargo source checksum: {error}")))?;
    if checksum.get("package").and_then(serde_json::Value::as_str) != Some(archive_sha256.as_str())
    {
        return Err(MgError::Other(
            "package digest differs from the MGC lock".to_string(),
        ));
    }
    let listed_files = checksum
        .get("files")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| MgError::Other("Cargo checksum has no files map".to_string()))?
        .iter()
        .map(|(path, digest)| {
            digest
                .as_str()
                .map(|digest| (path.clone(), digest.to_string()))
                .ok_or_else(|| {
                    MgError::Other("Cargo checksum file digest is not a string".to_string())
                })
        })
        .collect::<MgResult<BTreeMap<_, _>>>()?;
    let actual_files = hash_source_files(package_root)?;
    if listed_files != expected_files
        || !source_files_match_archive(package_root, &expected_files, &actual_files)?
    {
        return Err(MgError::Other(
            "extracted package files differ from the MGC-verified source record".to_string(),
        ));
    }
    Ok(())
}
