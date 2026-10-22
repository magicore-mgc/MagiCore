//! `shared_cache_lookup.rs` — Shared resolution-cache lookup / tra cứu shared resolution-cache.
use super::super::super::WebAdapter;
use crate::normalize_resolved_graph;
use crate::profile::ResolveProfile;
use mgc_types::{Manifest, MgResult, adapter::ResolvedGraph};
use std::time::Instant;

#[allow(clippy::too_many_arguments)] // Explicit arguments keep policy gates and cache identity visible.
pub(in crate::adapter::resolution) fn read_shared_graph(
    adapter: &WebAdapter,
    manifest: &Manifest,
    resolved_manifest: &Manifest,
    allow_shortcircuit: bool,
    age_gate_active: bool,
    resolution_cache_key: Option<&str>,
    profile: &mut ResolveProfile,
    started_at: Instant,
) -> MgResult<Option<ResolvedGraph>> {
    // Preserve the edition-2024 prefetch/cache condition — giữ nguyên điều kiện prefetch/cache edition 2024.
    if shared_cache_reuse_allowed(allow_shortcircuit, age_gate_active)
        && let (Some(shared_cache), Some(key)) =
            (adapter.shared_cache.as_ref(), resolution_cache_key)
        && let Some(graph) = shared_cache.read_resolution(key, &adapter.registry_url)?
        && has_exact_direct_root_selections(resolved_manifest, &graph)
    {
        let graph = normalize_resolved_graph(resolved_manifest, graph);
        crate::supply_chain::enforce_cached_graph_downgrade_guard(&graph, manifest)?;
        profile.mark("shared_resolution_cache_hit", started_at);
        profile.flush(started_at.elapsed().as_millis() as u64);
        return Ok(Some(graph));
    }
    profile.mark("shared_resolution_cache_check", started_at);
    Ok(None)
}

fn shared_cache_reuse_allowed(allow_shortcircuit: bool, age_gate_active: bool) -> bool {
    allow_shortcircuit && !age_gate_active
}

/// Reject cache entries that would require inferring a direct root by name/range.
/// Từ chối cache cần đoán root trực tiếp theo tên/range.
fn has_exact_direct_root_selections(manifest: &Manifest, graph: &ResolvedGraph) -> bool {
    manifest.all_dependencies().all(|dependency| {
        let mut candidates = graph.packages.iter().filter(|package| {
            package.direct
                && package.id.name() == &dependency.name
                && crate::is_manifest_root_package(manifest, &package.id)
        });
        candidates.next().is_some() && candidates.next().is_none()
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
