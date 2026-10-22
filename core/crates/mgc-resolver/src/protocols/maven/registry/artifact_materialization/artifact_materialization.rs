//! Write verified Maven artifact bytes into the local repository layout.
//! Ghi bytes artifact Maven đã xác minh vào layout repository local.
use super::super::coordinate_validation::validate_maven_cache_component;
use crate::protocols::ResolvedEntry;
use crate::protocols::maven::registry::registry::MavenProtocol;
use mgc_types::MgResult;
use std::path::PathBuf;

impl MavenProtocol {
    /// `{m2_root}/repository/{gpath}/{artifact}/{version}/` (jar + pom +
    /// checksum công bố) — đọc được bởi `mvn -o`.
    pub fn materialize(
        &self,
        entry: &ResolvedEntry,
        jar_bytes: &[u8],
        pom_bytes: &[u8],
        m2_root: &std::path::Path,
    ) -> MgResult<PathBuf> {
        let (group, artifact) = Self::split_coordinate(&entry.name)?;
        validate_maven_cache_component(&entry.version, "version")?;
        let gpath = group.replace('.', "/");
        let version = &entry.version;
        let dir = m2_root
            .join("repository")
            .join(gpath)
            .join(&artifact)
            .join(version);
        std::fs::create_dir_all(&dir)?;
        let jar_name = format!("{artifact}-{version}.jar");
        let pom_name = format!("{artifact}-{version}.pom");
        std::fs::write(dir.join(&jar_name), jar_bytes)?;
        std::fs::write(dir.join(&pom_name), pom_bytes)?;
        // Write the checksums we actually verified — mvn -o re-checks them.
        // (Viết đúng checksum mình đã xác minh — mvn -o kiểm tra lại.)
        if entry.sha256.is_empty() {
            // sha1-only provenance recorded in markers.
            // (Nguồn sha1-only được ghi trong marker.)
            if let Some(marker) = entry
                .extra_markers
                .iter()
                .find(|m| m.starts_with("checksum-sha1:"))
            {
                std::fs::write(
                    dir.join(format!("{jar_name}.sha1")),
                    marker.trim_start_matches("checksum-sha1:"),
                )?;
            }
        } else {
            std::fs::write(dir.join(format!("{jar_name}.sha256")), &entry.sha256)?;
        }
        Ok(dir.join(jar_name))
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
