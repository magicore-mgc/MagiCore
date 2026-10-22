//! Maven manifest writer.
//! Writer manifest Maven.

use super::super::prune_pom_dependencies::prune_pom_dependencies;
use super::super::upsert_pom_dependency::upsert_pom_dependency;
use mgc_types::{Manifest, MgResult};
use std::path::Path;

/// Write pom.xml dependency pins from the manifest (native add). mgc owns
/// the compile/runtime `<dependency>` entries: each manifest dep becomes
/// (or updates) a block inside top-level `<dependencies>` (created when
/// absent). dependencyManagement/profiles/plugins are never touched.
/// A non-exact range fails closed — callers pin via resolve-first.
/// (Ghi dependency pom.xml từ manifest.)
pub(crate) fn write_pom_manifest(root: &Path, manifest: &Manifest) -> MgResult<()> {
    let path = root.join("pom.xml");
    let content = mgc_adapter_base::project_file::read_regular_text(&path, "pom.xml")?;
    let mut pins: Vec<(String, String, String)> = Vec::new();
    for dep in manifest.all_dependencies() {
        let version = dep
            .range
            .satisfying_version()
            .map(|v| v.to_string())
            .unwrap_or_else(|| dep.range.to_string());
        let clean = version.trim_start_matches(['=', 'v', ' ']);
        mgc_types::Version::parse(clean).map_err(|_| {
            mgc_types::MgError::Other(format!(
                "pom writer needs an exact version for '{}', got '{}' — resolve-first must pin before save",
                dep.name.as_str(),
                dep.range
            ))
        })?;
        let mut parts = dep.name.as_str().splitn(2, ':');
        let (group, artifact) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
        if group.is_empty() || artifact.is_empty() {
            return Err(mgc_types::MgError::Other(format!(
                "pom writer needs group:artifact coordinates, got '{}'",
                dep.name.as_str()
            )));
        }
        pins.push((group.to_string(), artifact.to_string(), clean.to_string()));
    }
    pins.sort();
    pins.dedup();
    let mut out = content;
    for (group, artifact, version) in &pins {
        out = upsert_pom_dependency(&out, group, artifact, version)?;
    }
    // Prune stale top-level pins (remove honesty): dependency blocks
    // whose G:A is NOT in the manifest are deleted. A remove that leaves
    // the pin behind is a lie.
    // (Xóa dependency không còn trong manifest.)
    out = prune_pom_dependencies(&out, &pins);
    mgc_adapter_base::project_file::atomic_write_regular(&path, out.as_bytes())?;
    Ok(())
}
