//! NuGet version choice, registration integrity, dependency and TFM graph resolution.
//! Chọn version NuGet, kiểm tra integrity registration và resolve graph dependency/TFM.

use super::dependency_graph::resolve_dependencies;
use super::registration_integrity::catalog_sha512_marker;
use super::version_selection::select_listed_version;
use crate::protocols::nuget::{NuGetProtocol, nuget_matches};
use crate::protocols::{RegistryProtocol, ResolvedEntry};
use async_trait::async_trait;
use base64::Engine as _;
use mgc_types::{MgError, MgResult, Version};
use sha2::{Digest, Sha512};

#[async_trait]
impl RegistryProtocol for NuGetProtocol {
    fn matches_requirement(&self, range: &str, version: &Version) -> bool {
        nuget_matches(range, version)
    }

    async fn resolve(&self, name: &str, range: &str) -> MgResult<ResolvedEntry> {
        let id = name.trim();
        let id_lower = id.to_lowercase();
        let version = select_listed_version(self, id, &id_lower, range).await?;
        let mut markers = vec![catalog_sha512_marker(self, id, &id_lower, &version).await?];
        let graph = resolve_dependencies(self, id, &version, self.consumer_tfm.as_deref()).await?;
        markers.extend(graph.markers);

        let artifact_url = self.nupkg_url(id, &version);
        Ok(ResolvedEntry {
            name: id.to_string(),
            version,
            deps: graph.dependencies,
            artifact_url,
            // sha256 stays empty — the authoritative digest is SHA-512 and
            // rides the marker (verify override).
            // (sha256 giữ rỗng — digest chính thống là SHA-512 và nằm trong
            // marker (verify override).)
            sha256: String::new(),
            extra_markers: markers,
        })
    }

    async fn download(&self, entry: &ResolvedEntry) -> MgResult<Vec<u8>> {
        self.get_bytes(&entry.artifact_url).await
    }

    /// Verify overrides the shared default: the registration's base64
    /// SHA-512 (marker `sha512:…`) is the authoritative check — a nupkg
    /// with no recorded hash is never installed.
    /// (Verify ghi đè mặc định: SHA-512 base64 của registration (marker
    /// `sha512:…`) là kiểm tra chính thống — nupkg không có hash đã ghi
    /// không bao giờ được cài.)
    fn verify(&self, entry: &ResolvedEntry, bytes: &[u8]) -> MgResult<()> {
        let marker = entry
            .extra_markers
            .iter()
            .find(|m| m.starts_with("sha512:"))
            .ok_or_else(|| {
                MgError::Integrity(format!(
                    "no integrity source for {}@{} (no registration SHA-512) — fail-closed",
                    entry.name, entry.version
                ))
            })?;
        let expected = marker.trim_start_matches("sha512:");
        let actual = base64::engine::general_purpose::STANDARD.encode(Sha512::digest(bytes));
        if actual != expected {
            return Err(MgError::Integrity(format!(
                "sha512 mismatch for {}@{}",
                entry.name, entry.version
            )));
        }
        Ok(())
    }
}
