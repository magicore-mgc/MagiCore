//! Download, verify, CAS-import and materialize the Android Maven subset.
//! Tải, xác minh, import CAS và materialize tập Maven Android.

use mgc_resolver::protocols::{MavenProtocol, RegistryProtocol};
use mgc_types::{MgError, MgResult, ResolvedGraph};
use std::path::PathBuf;

/// Install Maven artifacts into the MGC-managed local repository.
/// Cài artifact Maven vào repository cục bộ do MGC quản lý.
pub(in crate::native::rn_layers) async fn install_maven_subset(
    graph: &ResolvedGraph,
) -> MgResult<mgc_types::adapter::InstallSummary> {
    let started = std::time::Instant::now();
    let protocol = MavenProtocol::from_env();
    let m2_root = maven_store_root()?;
    let store = mgc_store::ContentStore::new(mgc_store::default_store_root())
        .map_err(|e| MgError::Store(e.to_string()))?;
    let mut added = Vec::with_capacity(graph.packages.len());
    for package in &graph.packages {
        let entry = mgc_resolver::protocols::ResolvedEntry {
            name: package.id.name_str().to_string(),
            version: package.id.version().to_string(),
            deps: Vec::new(),
            artifact_url: package.tarball_url.clone(),
            sha256: mgc_lockfile::policy::native_sha256_hex(&package.integrity),
            extra_markers: Vec::new(),
        };
        let jar = protocol.download(&entry).await?;
        protocol.verify(&entry, &jar)?;
        let (group, artifact) = MavenProtocol::split_coordinate(&entry.name)?;
        let pom = protocol
            .download_pom(&group, &artifact, &entry.version)
            .await?;
        store
            .import_bytes(&jar)
            .map_err(|e| MgError::Store(e.to_string()))?;
        store
            .import_bytes(&pom)
            .map_err(|e| MgError::Store(e.to_string()))?;
        protocol.materialize(&entry, &jar, &pom, &m2_root)?;
        added.push(package.id.clone());
    }
    Ok(mgc_types::adapter::InstallSummary {
        added,
        bytes_from_cache: 0,
        duration_ms: started.elapsed().as_millis() as u64,
        cache_mode: mgc_types::adapter::InstallCacheMode::MgCStore,
    })
}

fn maven_store_root() -> MgResult<PathBuf> {
    let root = maven_store_root_path(std::env::var("MGC_MAVEN_STORE_ROOT").ok());
    std::fs::create_dir_all(&root)
        .map_err(|e| MgError::Other(format!("create maven store root: {e}")))?;
    Ok(root)
}

fn maven_store_root_path(override_root: Option<String>) -> PathBuf {
    match override_root.filter(|value| !value.trim().is_empty()) {
        Some(value) => PathBuf::from(value),
        None => mgc_platform::paths::mgc_data_dir()
            .join("store")
            .join("maven"),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
