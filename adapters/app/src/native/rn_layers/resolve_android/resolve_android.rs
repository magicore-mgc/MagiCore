//! Convert exact Gradle pins to a Maven manifest and merge its graph/locks.
//! Chuyển pin Gradle chính xác thành manifest Maven và gộp graph/lock.

use crate::native::rn_layers::{RNResolution, lock_markers::tag_tier};
use mgc_lib_adapter::native::engine::resolve_with_protocol;
use mgc_lockfile::EcosystemTag;
use mgc_resolver::protocols::MavenProtocol;
use mgc_types::{
    DependencySpec, Ecosystem, Manifest, MgError, MgResult, PackageName, ResolvedGraph,
    VersionRange,
};
use std::path::Path;

/// Resolve the Android Maven tier when `gradle.lockfile` exists.
/// Resolve tier Maven Android nếu có `gradle.lockfile`.
pub(in crate::native::rn_layers) async fn resolve_android(
    project_root: &Path,
) -> MgResult<RNResolution> {
    let mut result = RNResolution {
        graph: ResolvedGraph::empty(),
        lock_packages: Vec::new(),
    };
    let gradle_lock = project_root.join("gradle.lockfile");
    if !gradle_lock.is_file() {
        return Ok(result);
    }

    let text = std::fs::read_to_string(&gradle_lock)
        .map_err(|e| MgError::Other(format!("read gradle.lockfile: {e}")))?;
    let pins = mgc_resolver::protocols::reactnative::collapse_gradle_pins(
        &mgc_resolver::protocols::reactnative::parse_gradle_lockfile(&text)?,
    )?;
    if pins.is_empty() {
        return Ok(result);
    }

    let android_manifest = android_manifest_from_pins(&pins)?;
    let maven = MavenProtocol::from_env();
    let resolution = resolve_with_protocol(
        &maven,
        EcosystemTag::Maven,
        "maven://repo.maven.apache.org",
        &android_manifest,
    )
    .await?;
    result.graph.packages.extend(resolution.graph.packages);
    result.lock_packages = resolution
        .lock_packages
        .into_iter()
        .map(|package| tag_tier(package, "android"))
        .collect();
    Ok(result)
}

fn android_manifest_from_pins(pins: &[(String, String)]) -> MgResult<Manifest> {
    let mut manifest = Manifest::new("rn-android", Ecosystem::App);
    for (coordinate, version) in pins {
        let name = PackageName::new(coordinate.clone())
            .map_err(|e| MgError::Other(format!("invalid Maven coordinate {coordinate}: {e}")))?;
        let range = VersionRange::parse(&exact_maven_version_range(version))?;
        manifest.add_dep(DependencySpec::new(name, range), false, false, false);
    }
    Ok(manifest)
}

fn exact_maven_version_range(version: &str) -> String {
    format!("[{version}]")
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
