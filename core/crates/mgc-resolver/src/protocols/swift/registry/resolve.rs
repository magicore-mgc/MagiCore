//! Swift registry version, checksum, and archive resolution. / Resolve version, checksum và archive registry Swift.

use crate::protocols::ResolvedEntry;
use crate::protocols::swift::archive_dependencies::swift_deps_from_archive;
use crate::protocols::swift::version_requirement::swift_matches;
use mgc_types::{MgError, MgResult, Version};
use serde_json::Value;

use super::registry::{
    MAX_SWIFT_ARCHIVE_BYTES, MAX_SWIFT_CHECKSUM_BYTES, MAX_SWIFT_VERSION_LIST_BYTES,
    SwiftRegistryProtocol,
};

impl SwiftRegistryProtocol {
    /// Registry-path resolve (name is `scope/name`).
    /// Resolve đường registry (name là `scope/name`).
    pub(super) async fn resolve_registry(
        &self,
        name: &str,
        range: &str,
    ) -> MgResult<ResolvedEntry> {
        if self.registry_base.is_empty() {
            return Err(MgError::Other(
                "no Swift registry configured — set MGC_SWIFT_REGISTRY_URL (fail-closed)"
                    .to_string(),
            ));
        }
        // Registry packages REQUIRE the scope/name form (SwiftPM registries
        // are scoped; a bare name cannot be addressed).
        // (Package registry BẮT BUỘC dạng scope/name (registry SwiftPM có
        // scope; tên trần không định vị được).)
        let Some((scope, _pkg)) = name.split_once('/') else {
            return Err(MgError::Other(format!(
                "Swift registry packages require a scope/name identifier — '{name}' has no scope (fail-closed)"
            )));
        };
        let versions_url = self.registry_url_for(name, &format!("{scope}.json"));
        let (status, body) = {
            let (s, b) = self
                .get_bytes(&versions_url, MAX_SWIFT_VERSION_LIST_BYTES)
                .await?;
            (s, String::from_utf8_lossy(&b).into_owned())
        };
        if !(200..300).contains(&status) {
            return Err(MgError::Network(format!(
                "GET {versions_url} returned {status} — no versions for {name} (fail-closed)"
            )));
        }
        let doc: Value = serde_json::from_str(&body)
            .map_err(|e| MgError::Other(format!("parse swift version list failed: {e}")))?;
        let versions: Vec<&str> = doc
            .get("versions")
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();

        // SwiftPM selection: the HIGHEST matching version wins.
        // (Selection SwiftPM: version CAO NHẤT khớp thắng.)
        let mut best: Option<(Version, String)> = None;
        for raw in &versions {
            let Ok(v) = Version::parse(raw) else {
                continue;
            };
            if swift_matches(range, &v) && best.as_ref().is_none_or(|(bv, _)| v > *bv) {
                best = Some((v, (*raw).to_string()));
            }
        }
        let version = best
            .ok_or_else(|| MgError::Other(format!("no version of {name} matches range '{range}'")))?
            .1;

        // Mandatory registry checksum (hex sha256 of the zip) — fetched
        // BEFORE the archive; a registry without it is fail-closed.
        // (Checksum registry bắt buộc (hex sha256 của zip) — tải TRƯỚC
        // archive; registry thiếu là fail-closed.)
        let sha_url = self.registry_url_for(name, &format!("{version}.sha256"));
        let (sha_status, sha_body) = self.get_bytes(&sha_url, MAX_SWIFT_CHECKSUM_BYTES).await?;
        if !(200..300).contains(&sha_status) {
            return Err(MgError::Integrity(format!(
                "GET {sha_url} returned {sha_status} — the registry publishes no checksum \
                 for {name} {version} (fail-closed)"
            )));
        }
        let sha256 = String::from_utf8_lossy(&sha_body)
            .trim()
            .to_ascii_lowercase();
        if sha256.len() != 64 || !sha256.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(MgError::Integrity(format!(
                "malformed .sha256 for {name} {version}: '{sha256}'"
            )));
        }

        // Source archive (cached for the install download).
        // (Archive nguồn (cache cho download lúc install).)
        let zip_url = self.registry_url_for(name, &format!("{version}.zip"));
        let (zip_status, zip_bytes) = self.get_bytes(&zip_url, MAX_SWIFT_ARCHIVE_BYTES).await?;
        if !(200..300).contains(&zip_status) {
            return Err(MgError::Network(format!(
                "GET {zip_url} returned {zip_status} (fail-closed)"
            )));
        }
        // Transitive deps come from the Package.swift at the archive root.
        // (Dep bắc cầu lấy từ Package.swift ở gốc archive.)
        let deps = swift_deps_from_archive(&zip_bytes)?;
        self.zip_cache
            .insert(format!("{name}@{version}"), zip_bytes)?;

        Ok(ResolvedEntry {
            name: name.to_string(),
            version,
            deps,
            artifact_url: zip_url,
            sha256,
            extra_markers: vec![format!("swift-registry:{scope}")],
        })
    }

    /// Git URLs are parsed for manifest compatibility but are not fetched via
    /// an external Git executable. Native HTTP Git transport is not shipped.
    /// URL Git được parse để tương thích manifest nhưng không fetch bằng Git
    /// executable bên ngoài; native HTTP Git transport chưa được triển khai.
    pub(super) async fn resolve_git(&self, name: &str, _range: &str) -> MgResult<ResolvedEntry> {
        Err(MgError::Unsupported {
            core: "app",
            capability: "native Swift Git dependency transport",
            guidance: format!(
                "Swift dependency '{name}' uses a Git source; MagiCore refuses to spawn Git until native Git transport is implemented"
            ),
        })
    }
}
