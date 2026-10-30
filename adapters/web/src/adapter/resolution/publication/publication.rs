//! `publication.rs` — Resolved graph publication / công bố graph đã resolve.
use super::super::super::WebAdapter;
use crate::cache::resolve_prefetch_enabled;
use crate::normalize_resolved_graph;
use crate::prefetch::spawn_tarball_download;
use crate::profile::ResolveProfile;
use mgc_types::{
    Manifest, MgResult,
    adapter::{ResolvedGraph, ResolvedPackage},
};
use std::time::Instant;

pub(in crate::adapter::resolution) fn publish(
    adapter: &WebAdapter,
    resolved_manifest: &Manifest,
    packages: Vec<ResolvedPackage>,
    resolution_cache_key: Option<&str>,
    profile: &mut ResolveProfile,
    started_at: Instant,
) -> MgResult<ResolvedGraph> {
    // A package a project source claims must be attributable to that source;
    // otherwise the registry chain served it and the claim is a promise the
    // install cannot keep (SEC-LOOP-04, dependency confusion).
    // (Package được source của project tuyên bố phải quy được về source đó;
    // nếu không, chain đã phục vụ nó và claim là lời hứa install không giữ được.)
    crate::supply_chain::ensure_project_source_claims(&packages, &adapter.registry_url)?;
    // Preserve the edition-2024 prefetch/cache condition — giữ nguyên điều kiện prefetch/cache edition 2024.
    if resolve_prefetch_enabled()
        && let Some(shared_cache) = adapter.shared_cache.clone()
    {
        let registry_url = adapter.registry_url.clone();
        *adapter.prefetch_handle_guard() = Some(spawn_tarball_download(
            shared_cache,
            packages.clone(),
            registry_url,
        ));
    }
    let graph = normalize_resolved_graph(resolved_manifest, ResolvedGraph { packages });
    if let (Some(shared_cache), Some(key)) = (adapter.shared_cache.as_ref(), resolution_cache_key) {
        let _ = shared_cache.write_resolution(key, &adapter.registry_url, &graph);
    }
    profile.mark("write_resolution_cache", started_at);
    profile.flush(started_at.elapsed().as_millis() as u64);
    Ok(graph)
}
