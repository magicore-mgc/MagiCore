use super::super::pom_dependencies::collect_pom_dependencies;
use super::super::pom_properties::{collect_managed_versions, collect_pom_properties};
use super::super::pom_xml::element_blocks;
use super::super::registry::MavenProtocol;
use super::super::version_requirement::maven_matches;
use crate::protocols::{RegistryProtocol, ResolvedEntry};
use async_trait::async_trait;
use mgc_types::{MgError, MgResult, Version};
use sha1::Sha1;
use sha2::{Digest, Sha256};

#[async_trait]
impl RegistryProtocol for MavenProtocol {
    fn matches_requirement(&self, range: &str, version: &Version) -> bool {
        maven_matches(range, version)
    }

    async fn resolve(&self, name: &str, range: &str) -> MgResult<ResolvedEntry> {
        let (group, artifact) = Self::split_coordinate(name)?;

        // maven-metadata.xml → published versions.
        // (maven-metadata.xml → các version đã công bố.)
        let metadata_url = self.repo_url_for(&group, &artifact, "", "maven-metadata.xml");
        let (status, body) = self.get_text(&metadata_url).await?;
        if !(200..300).contains(&status) {
            return Err(MgError::Network(format!(
                "GET {metadata_url} returned {status} — no version metadata for {name} (fail-closed)"
            )));
        }
        let mut best: Option<(Version, String)> = None;
        for versions_block in element_blocks(&body, "versions") {
            for v in element_blocks(versions_block, "version") {
                let raw = v.trim();
                let Ok(version) = Version::parse(raw) else {
                    continue;
                };
                if !maven_matches(range, &version) {
                    continue;
                }
                // Tie-break by raw string so distinct Maven qualifiers
                // (1.2.3.Final vs 1.2.3) stay deterministic.
                // (Tie-break theo chuỗi thô để qualifier Maven khác nhau
                // (1.2.3.Final vs 1.2.3) vẫn tất định.)
                if best
                    .as_ref()
                    .is_none_or(|(bv, br)| version > *bv || (version == *bv && raw > br.as_str()))
                {
                    best = Some((version, raw.to_string()));
                }
            }
        }
        let (_, version) = best.ok_or_else(|| {
            MgError::Other(format!("no version of {name} matches range '{range}'"))
        })?;

        let pom_url = self.repo_url_for(
            &group,
            &artifact,
            &version,
            &format!("{artifact}-{version}.pom"),
        );
        let (pom_status, pom) = self.get_text(&pom_url).await?;
        if !(200..300).contains(&pom_status) {
            return Err(MgError::Network(format!(
                "GET {pom_url} returned {pom_status} — the POM is required to build the graph (fail-closed)"
            )));
        }

        let mut markers = Vec::new();
        let (deps_xml, packaging) = collect_pom_dependencies(&pom)?;
        // Own-POM knowledge first; parent POMs merge lazily below, child
        // wins on every key (standard Maven inheritance).
        // (Tri thức POM hiện tại trước; POM cha merge lười bên dưới, con
        // thắng mọi key.)
        let mut props = collect_pom_properties(&pom)?;
        let mut managed = collect_managed_versions(&pom)?;
        let mut visited_parents: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        let mut deps = Vec::new();
        if packaging.as_deref() == Some("pom") {
            // BOM: no transitive graph — the main lock pins the BOM itself.
            // (BOM: không graph bắc cầu — lock chính ghim chính BOM.)
            markers.push("bom-packaging".to_string());
        } else {
            for dep in &deps_xml {
                let (Some(g), Some(a)) = (&dep.group, &dep.artifact) else {
                    continue;
                };
                let scope = dep.scope.as_deref().unwrap_or("compile");
                // Only compile/runtime scopes enter the graph.
                // (Chỉ scope compile/runtime vào graph.)
                if !matches!(scope, "compile" | "runtime" | "") {
                    markers.push(format!("scope-skip:{scope}:{g}:{a}"));
                    continue;
                }
                if dep.optional {
                    markers.push(format!("optional:{g}:{a}"));
                    continue;
                }
                let version = self
                    .maven_dep_version(
                        &pom,
                        g,
                        a,
                        dep.version.as_deref(),
                        &mut props,
                        &mut managed,
                        &mut visited_parents,
                    )
                    .await?;
                deps.push((format!("{g}:{a}"), version));
            }
        }

        // Jar checksum: .sha256 preferred, .sha1 fallback, neither →
        // fail-closed Integrity.
        // (Checksum jar: .sha256 ưu tiên, .sha1 fallback, không có gì →
        // fail-closed Integrity.)
        let jar_name = format!("{artifact}-{version}.jar");
        let sha256_url =
            self.repo_url_for(&group, &artifact, &version, &format!("{jar_name}.sha256"));
        let sha1_url = self.repo_url_for(&group, &artifact, &version, &format!("{jar_name}.sha1"));
        let (sha256_status, sha256_body) = self.get_text(&sha256_url).await?;
        let sha256 = if (200..300).contains(&sha256_status) {
            let hex = sha256_body.trim().to_ascii_lowercase();
            if hex.len() != 64 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(MgError::Integrity(format!(
                    "malformed .jar.sha256 for {name}:{version}: '{hex}'"
                )));
            }
            hex
        } else {
            let (sha1_status, sha1_body) = self.get_text(&sha1_url).await?;
            if !(200..300).contains(&sha1_status) {
                return Err(MgError::Integrity(format!(
                    "no .sha256/.sha1 published for {name}:{version} jar — cannot verify (fail-closed)"
                )));
            }
            let hex = sha1_body.trim().to_ascii_lowercase();
            if hex.len() != 40 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(MgError::Integrity(format!(
                    "malformed .jar.sha1 for {name}:{version}: '{hex}'"
                )));
            }
            markers.push(format!("checksum-sha1:{hex}"));
            String::new()
        };

        let jar_url = self.repo_url_for(&group, &artifact, &version, &jar_name);
        Ok(ResolvedEntry {
            name: name.to_string(),
            version,
            deps,
            artifact_url: jar_url,
            sha256,
            extra_markers: markers,
        })
    }

    async fn download(&self, entry: &ResolvedEntry) -> MgResult<Vec<u8>> {
        self.get_bytes(&entry.artifact_url).await
    }

    /// Verify overrides the shared default: sha256 when published, the
    /// recorded sha1 marker otherwise, and fail-closed when neither is
    /// available (an unverified jar is never installed).
    /// (Verify ghi đè mặc định: sha256 khi có, marker sha1 đã ghi nếu
    /// ngược lại, fail-closed khi không có cái nào — jar chưa xác minh
    /// không bao giờ được cài.)
    fn verify(&self, entry: &ResolvedEntry, bytes: &[u8]) -> MgResult<()> {
        if !entry.sha256.is_empty() {
            let actual = hex_sha256(bytes);
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
            .find(|m| m.starts_with("checksum-sha1:"))
            .ok_or_else(|| {
                MgError::Integrity(format!(
                    "no integrity source for {}@{} (no sha256, no sha1) — fail-closed",
                    entry.name, entry.version
                ))
            })?;
        let expected = marker.trim_start_matches("checksum-sha1:");
        let mut hasher = Sha1::new();
        hasher.update(bytes);
        let actual = hex::encode(hasher.finalize());
        if actual != expected {
            return Err(MgError::Integrity(format!(
                "sha1 mismatch for {}@{}: expected {expected}, got {actual}",
                entry.name, entry.version
            )));
        }
        Ok(())
    }
}

fn hex_sha256(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
