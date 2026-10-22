//! RegistryProtocol dispatch, download, and integrity verification. / Dispatch RegistryProtocol, download và xác minh integrity.

use crate::protocols::swift::git_source::is_git_name;
use crate::protocols::swift::version_requirement::swift_matches;
use crate::protocols::{RegistryProtocol, ResolvedEntry, sha256_hex};
use async_trait::async_trait;
use mgc_types::{MgError, MgResult, Version};

use super::registry::{MAX_SWIFT_ARCHIVE_BYTES, SwiftRegistryProtocol};

#[async_trait]
impl RegistryProtocol for SwiftRegistryProtocol {
    fn matches_requirement(&self, range: &str, version: &Version) -> bool {
        swift_matches(range, version)
    }

    async fn resolve(&self, name: &str, range: &str) -> MgResult<ResolvedEntry> {
        // Local paths never resolve anywhere: neither the git transport
        // (default-blocked) nor the scoped registry can serve them.
        // (Đường dẫn local không resolve ở đâu cả.)
        if name.starts_with("file://") {
            return Err(MgError::Other(
                "file:// dependencies are blocked — local paths never enter the git transport or the registry (fail-closed)".to_string(),
            ));
        }
        if is_git_name(name) {
            self.resolve_git(name, range).await
        } else {
            self.resolve_registry(name, range).await
        }
    }

    async fn download(&self, entry: &ResolvedEntry) -> MgResult<Vec<u8>> {
        if entry.extra_markers.iter().any(|m| m == "swift-git") {
            // Git checkouts are directories, not archive bytes — the
            // "download" is the pinned commit SHA (ASCII), the only
            // verifiable artifact identity (provenance, not payload).
            // (Checkout git là thư mục, không phải byte archive —
            // "download" là SHA commit đã ghim (ASCII), định danh artifact
            // duy nhất xác minh được (provenance, không phải payload).)
            return Ok(entry
                .extra_markers
                .iter()
                .find_map(|m| m.strip_prefix("git-commit:"))
                .unwrap_or_default()
                .as_bytes()
                .to_vec());
        }
        let key = format!("{}@{}", entry.name, entry.version);
        if let Some(bytes) = self.zip_cache.get(&key)? {
            return Ok(bytes);
        }
        let (status, bytes) = self
            .get_bytes(&entry.artifact_url, MAX_SWIFT_ARCHIVE_BYTES)
            .await?;
        if !(200..300).contains(&status) {
            return Err(MgError::Network(format!(
                "GET {} returned {status}",
                entry.artifact_url
            )));
        }
        Ok(bytes)
    }

    /// Verify overrides the shared default for GIT entries: the downloaded
    /// "artifact" is the pinned commit SHA — it must equal the resolve-time
    /// marker (a moved ref fails closed). Registry entries keep the shared
    /// sha256 check (the registry checksum is mandatory).
    /// (Verify ghi đè mặc định cho entry GIT: "artifact" tải về là SHA
    /// commit đã ghim — phải bằng marker lúc resolve (ref bị dịch là
    /// fail-closed). Entry registry giữ kiểm tra sha256 chung (checksum
    /// registry là bắt buộc).)
    fn verify(&self, entry: &ResolvedEntry, bytes: &[u8]) -> MgResult<()> {
        if entry.extra_markers.iter().any(|m| m == "swift-git") {
            let expected = entry
                .extra_markers
                .iter()
                .find_map(|m| m.strip_prefix("git-commit:"))
                .ok_or_else(|| {
                    MgError::Integrity(format!(
                        "no git-commit provenance for {} @ {} (fail-closed)",
                        entry.name, entry.version
                    ))
                })?;
            let downloaded = String::from_utf8_lossy(bytes);
            let actual = downloaded.trim();
            if actual != expected {
                return Err(MgError::Integrity(format!(
                    "git provenance mismatch for {} @ {}: expected {expected}, got {actual}",
                    entry.name, entry.version
                )));
            }
            return Ok(());
        }
        // Registry entries carry a mandatory checksum: an empty digest
        // is a corrupt registry record, failed closed like the shared
        // default (V1.2 zero-trust).
        // (Entry registry mang checksum bắt buộc: digest rỗng là bản ghi
        // registry hỏng, fail-closed như default chung.)
        if entry.sha256.is_empty() {
            return Err(MgError::Integrity(format!(
                "refusing artifact without digest for {}@{} (Swift registry checksum missing)",
                entry.name, entry.version
            )));
        }
        let actual = sha256_hex(bytes);
        if !actual.eq_ignore_ascii_case(&entry.sha256) {
            return Err(MgError::Integrity(format!(
                "sha256 mismatch for {}@{}: expected {}, got {}",
                entry.name, entry.version, entry.sha256, actual
            )));
        }
        Ok(())
    }
}
