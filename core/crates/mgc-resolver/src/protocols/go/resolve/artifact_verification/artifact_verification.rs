//! Verify Go artifacts with proxy sha256 or sumdb h1 directory hashes.
//! Xác minh artifact Go bằng sha256 proxy hoặc directory hash h1 sumdb.

use crate::protocols::go::{GoModProtocol, dirhash_zip};
use crate::protocols::{ResolvedEntry, sha256_hex, zip_reader};
use mgc_types::{MgError, MgResult};

impl GoModProtocol {
    /// Verify the declared hash or fail closed when no verifiable source exists.
    /// Xác minh hash đã khai báo hoặc fail-closed khi không có nguồn xác minh.
    pub(in crate::protocols::go::resolve) fn verify_artifact(
        &self,
        entry: &ResolvedEntry,
        bytes: &[u8],
    ) -> MgResult<()> {
        if !entry.sha256.is_empty() {
            let actual = sha256_hex(bytes);
            if !actual.eq_ignore_ascii_case(&entry.sha256) {
                return Err(MgError::Integrity(format!(
                    "sha256 mismatch for {}@{}: expected {}, got {}",
                    entry.name, entry.version, entry.sha256, actual
                )));
            }
            return Ok(());
        }
        let marker = entry
            .extra_markers
            .iter()
            .find(|marker| marker.starts_with("sumdb-ziphash:h1:"))
            .ok_or_else(|| {
                MgError::Integrity(format!(
                    "no integrity source for {}@{} (no .ziphash, no sumdb record) — fail-closed",
                    entry.name, entry.version
                ))
            })?;
        let expected = marker.trim_start_matches("sumdb-ziphash:");
        let actual = dirhash_zip(&zip_reader::read_zip_entries(bytes)?)?;
        if actual != expected {
            return Err(MgError::Integrity(format!(
                "sumdb dirhash mismatch for {}@{}: expected {expected}, got {actual}",
                entry.name, entry.version
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
