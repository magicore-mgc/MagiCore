//! `lockfile_lookup.rs` — Lockfile graph lookup / tra cứu graph lockfile.
use super::super::v4_short_circuit::try_v4_short_circuit;
use crate::lockfile::{
    build_graph_from_lockfile, lockfile_satisfies_manifest, read_web_lockfile_checked,
};
use crate::normalize_resolved_graph;
use crate::profile::ResolveProfile;
use mgc_types::{Manifest, MgResult, adapter::ResolvedGraph};
use std::path::Path;
use std::time::Instant;

pub(in crate::adapter::resolution) fn find_lockfile_graph(
    resolved_manifest: &Manifest,
    allow_shortcircuit: bool,
    age_gate_active: bool,
    profile: &mut ResolveProfile,
    started_at: Instant,
) -> MgResult<Option<ResolvedGraph>> {
    // Lockfile short-circuit (skipped by resolve_fresh): a satisfying
    // lock reuses its graph instead of re-solving. Bumper flows must
    // never take it — the locked version is exactly what they escape.
    // (Short-circuit lockfile — bumper không bao giờ đi đường này.)
    // Preserve the edition-2024 prefetch/cache condition — giữ nguyên điều kiện prefetch/cache edition 2024.
    // v4 documents take the lossless v4 branch first (shared
    // v4_graph home); anything unusable there misses the short-circuit
    // and falls through to fresh resolve — EXCEPT corrupt v4 files,
    // which fail exactly like the legacy reader. Lossy-but-valid v4
    // is a miss here (the CLI authority path enforces its hardness
    // before the adapter ever runs inside install).
    if allow_shortcircuit
        && !age_gate_active
        && let Some(graph) = try_v4_short_circuit(resolved_manifest)?
    {
        let graph = normalize_resolved_graph(resolved_manifest, graph);
        profile.mark("lockfile_short_circuit_v4", started_at);
        profile.flush(started_at.elapsed().as_millis() as u64);
        return Ok(Some(graph));
    }
    // Legacy short-circuit runs only when the lock on disk is NOT v4:
    // a v4 miss above means re-resolve, and the legacy reader would
    // misparse a v4 document instead of missing cleanly.
    // (Short-circuit legacy chỉ chạy khi lock đĩa KHÔNG phải v4: v4 miss
    // ở trên nghĩa là resolve mới, reader legacy sẽ parse sai v4 thay vì
    // miss sạch.)
    let legacy_readable = !crate::lockfile::lock_on_disk_is_v4(std::path::Path::new("."));
    if allow_shortcircuit
        && !age_gate_active
        && legacy_readable
        && let Some(lockfile) = read_web_lockfile_checked(Path::new("."))?
        && lockfile_satisfies_manifest(&lockfile, resolved_manifest)
        && let Ok(Some(graph)) = build_graph_from_lockfile(
            &lockfile,
            resolved_manifest,
            &crate::lockfile::web_lock_owner_core(Path::new("."))?,
        )
    {
        let graph = normalize_resolved_graph(resolved_manifest, graph);
        profile.mark("lockfile_short_circuit", started_at);
        profile.flush(started_at.elapsed().as_millis() as u64);
        return Ok(Some(graph));
    }
    profile.mark("lockfile_check", started_at);
    Ok(None)
}
