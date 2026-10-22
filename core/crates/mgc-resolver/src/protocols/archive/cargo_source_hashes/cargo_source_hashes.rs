//! Hash validated Cargo archive files without writing them to disk.
//! Hash file trong archive Cargo đã xác minh mà không ghi ra đĩa.

use super::super::archive::{MAX_TAR_ENTRY_UNCOMPRESSED, MAX_TAR_TOTAL_UNCOMPRESSED};
use super::super::{ArchiveError, ArchiveResult};
use std::io::Read as _;
use std::path::{Component, Path};

/// Hash Cargo crate files directly from a verified archive without extracting them.
/// Tính hash file crate trực tiếp từ archive đã verify, không giải nén ra đĩa.
pub(in crate::protocols::archive) fn cargo_source_file_hashes(
    bytes: &[u8],
    expected_root: &str,
) -> ArchiveResult<std::collections::BTreeMap<String, String>> {
    use sha2::{Digest, Sha256};
    use std::collections::BTreeMap;

    let expected = Path::new(expected_root);
    if expected.components().count() != 1
        || !matches!(expected.components().next(), Some(Component::Normal(_)))
    {
        return Err(ArchiveError::Integrity(format!(
            "invalid Cargo crate archive root '{expected_root}'"
        )));
    }

    let decoder = flate2::read::GzDecoder::new(bytes);
    let mut archive = tar::Archive::new(decoder);
    let mut total_uncompressed = 0_u64;
    let mut saw_manifest = false;
    let mut hashes = BTreeMap::new();

    for entry in archive.entries()? {
        let mut entry = entry?;
        let archive_path =
            super::super::archive_path::sanitize_archive_path(entry.path()?.as_ref())?;
        let relative = archive_path.strip_prefix(expected).map_err(|_| {
            ArchiveError::Integrity(format!(
                "Cargo crate archive entry '{}' is outside expected root '{expected_root}'",
                archive_path.display()
            ))
        })?;
        let entry_type = entry.header().entry_type();
        if entry_type.is_symlink() || entry_type.is_hard_link() {
            return Err(ArchiveError::Integrity(format!(
                "Cargo crate archive links are not allowed: {}",
                archive_path.display()
            )));
        }
        if relative.as_os_str().is_empty() {
            if entry_type.is_dir() {
                continue;
            }
            return Err(ArchiveError::Integrity(format!(
                "Cargo crate archive root '{expected_root}' must be a directory"
            )));
        }
        if entry_type.is_dir() {
            continue;
        }
        if matches!(entry_type.as_byte(), b'g' | b'x') {
            continue;
        }
        if !entry_type.is_file() {
            return Err(ArchiveError::Integrity(format!(
                "unsupported Cargo crate archive entry type for {}",
                archive_path.display()
            )));
        }

        let size = entry.size();
        if size > MAX_TAR_ENTRY_UNCOMPRESSED {
            return Err(ArchiveError::Integrity(format!(
                "Cargo crate archive entry '{}' exceeds the per-entry size limit",
                archive_path.display()
            )));
        }
        total_uncompressed = total_uncompressed.checked_add(size).ok_or_else(|| {
            ArchiveError::Integrity("Cargo crate archive size overflow".to_string())
        })?;
        if total_uncompressed > MAX_TAR_TOTAL_UNCOMPRESSED {
            return Err(ArchiveError::Integrity(
                "Cargo crate archive exceeds the total uncompressed size limit".to_string(),
            ));
        }

        let mut parts = Vec::new();
        for component in relative.components() {
            let Component::Normal(part) = component else {
                return Err(ArchiveError::Integrity(
                    "Cargo crate archive path has an unsafe component".to_string(),
                ));
            };
            parts.push(part.to_str().ok_or_else(|| {
                ArchiveError::Integrity("Cargo crate archive path is not UTF-8".to_string())
            })?);
        }
        let relative_path = parts.join("/");
        if relative_path == "Cargo.toml" {
            saw_manifest = true;
        }
        let mut contents = Vec::with_capacity(size as usize);
        entry.read_to_end(&mut contents)?;
        if hashes
            .insert(relative_path.clone(), hex::encode(Sha256::digest(contents)))
            .is_some()
        {
            return Err(ArchiveError::Integrity(format!(
                "Cargo crate archive contains duplicate file '{relative_path}'"
            )));
        }
    }
    if !saw_manifest {
        return Err(ArchiveError::Integrity(format!(
            "Cargo crate archive '{expected_root}' has no root Cargo.toml"
        )));
    }
    Ok(hashes)
}
