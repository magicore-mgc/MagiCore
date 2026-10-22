//! Partition a merged graph and install each tier through its owning pipeline.
//! Chia graph gộp và cài từng tier qua pipeline sở hữu.

use crate::native::rn_layers::{graph_subset::subset, install_maven::install_maven_subset};
use mgc_lockfile::EcosystemTag;
use mgc_types::capabilities::ContentStoreProvider;
use mgc_types::{MgError, MgResult, PackageId, ResolvedGraph};
use std::collections::HashSet;
use std::path::Path;

/// Install Android, iOS and JavaScript package subsets using their existing owners.
/// Cài các tập package Android, iOS, JavaScript qua owner hiện tại.
pub async fn install_rn_layers(
    graph: &ResolvedGraph,
    lock_packages: &[mgc_lockfile::Package],
    project_root: &Path,
    opts: mgc_types::adapter::InstallOptions,
) -> MgResult<mgc_types::adapter::InstallSummary> {
    let maven_names: HashSet<&str> = lock_packages
        .iter()
        .filter(|package| package.ecosystem == EcosystemTag::Maven)
        .map(|package| package.name.as_str())
        .collect();
    let pod_names: HashSet<&str> = lock_packages
        .iter()
        .filter(|package| package.ecosystem == EcosystemTag::CocoaPods)
        .map(|package| package.name.as_str())
        .collect();

    let started = std::time::Instant::now();
    let mut added: Vec<PackageId> = Vec::new();
    let android_graph = subset(graph, |name| maven_names.contains(name));
    if !android_graph.packages.is_empty() {
        added.extend(install_maven_subset(&android_graph).await?.added);
    }

    for package in &graph.packages {
        if pod_names.contains(package.id.name_str()) {
            added.push(package.id.clone());
        }
    }

    let js_graph = subset(graph, |name| {
        !maven_names.contains(name) && !pod_names.contains(name)
    });
    let mut web_summary = None;
    if !js_graph.packages.is_empty() {
        let web = mgc_web_adapter::WebAdapter::new()
            .map_err(|e| MgError::Other(format!("web adapter construction failed: {e}")))?;
        let summary = ContentStoreProvider::install(&web, &js_graph, project_root, opts).await?;
        added.extend(summary.added.iter().cloned());
        web_summary = Some(summary);
    }

    Ok(mgc_types::adapter::InstallSummary {
        added,
        bytes_from_cache: web_summary
            .as_ref()
            .map_or(0, |summary| summary.bytes_from_cache),
        duration_ms: started.elapsed().as_millis() as u64,
        cache_mode: web_summary
            .map(|summary| summary.cache_mode)
            .unwrap_or(mgc_types::adapter::InstallCacheMode::MgCStore),
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
