//! `v4_writer.rs` — v4 lane merge, replay, signature guard and atomic publication.
//! merge lane v4, replay, cổng chữ ký và ghi atomic

use super::super::{LockfileError, read_v4_document_if_present, resolved_graphs_match_material};
use mgc_types::{MgResult, adapter::ResolvedGraph};
use std::path::Path;

/// v4 install write path: build the document straight from the resolved
/// graph (no lossy v3 detour), fill digest + timestamp, prove round-trip,
/// and atomically replace the lock. A matching v4 already on disk is kept
/// byte-identical (replay, not rewrite).
/// (Đường ghi v4 của install: dựng tài liệu thẳng từ graph đã resolve
/// (không vòng v3 hao hụt), điền digest + timestamp, chứng minh round-trip
/// và thay atomic. v4 khớp trên đĩa được giữ nguyên byte.)
pub(crate) fn write_web_lockfile_v4(
    project_root: &Path,
    lock_path: &Path,
    graph: &ResolvedGraph,
    registry_url: &str,
    owner_core: &str,
) -> MgResult<()> {
    // A matching v4 on disk stays untouched (replay parity with the v3
    // short-circuit below). Anything else is rebuilt from the graph.
    // (v4 khớp trên đĩa giữ nguyên (parity replay với short-circuit v3).)
    if let Some(doc) = read_v4_document_if_present(project_root)?
        && (doc.root_dependencies_by_owner.is_empty()
            || doc.root_dependencies_by_owner.contains_key(owner_core))
        && let Ok(replayed) = mgc_lockfile::v4_graph::graph_from_v4_lockfile(&doc, owner_core)
        && resolved_graphs_match_material(&replayed, graph)
    {
        return Ok(());
    }
    // Unreplayable verified bases are merged from the fresh graph, with signature guards below.
    // Base đã verify nhưng không replay được sẽ merge graph mới, vẫn qua cổng chữ ký bên dưới.
    // Signed v4 on disk is never rewritten without explicit re-signing;
    // unsigned v4 mutates through the schema-aware path (the legacy gate
    // below would refuse every v4 document, signed or not).
    // (v4 đã ký trên đĩa không bao giờ bị ghi đè nếu không ký lại tường
    // minh; v4 không ký mutate qua đường schema-aware (cổng legacy dưới
    // từ chối mọi v4, ký hay không).)
    if let Some(doc) = read_v4_document_if_present(project_root)? {
        mgc_lockfile::atomic::ensure_v4_unsigned_for_rewrite(&doc)
            .map_err(|error| LockfileError::Mutation(error.to_string()))?;
    }
    // Lane-first merge (parity with app/lib): sibling v4 rows on disk are
    // preserved; legacy rows cannot follow and are reported, never silent.
    // (Merge lane trước (parity với app/lib): giữ row v4 khác trên đĩa;
    // row legacy không theo được thì báo, không lặng.)
    let generator = format!("mgc/{}", env!("CARGO_PKG_VERSION"));
    let (base, had_legacy_rows) = mgc_lockfile::migrate::read_v4_merge_base(lock_path)
        .map_err(|error| LockfileError::Merge(error.to_string()))?;
    // Every row of this lane must point at a source that exists in the
    // document. A configured source that serves the configured registry
    // adopts the registry-derived id (so rows and the source table agree);
    // otherwise the lane falls back to its own registry record.
    // (Mọi row của lane phải trỏ tới source có trong document. Source cấu
    // hình phục vụ đúng registry thì nhận id suy từ registry.)
    let sources = crate::supply_chain::project_lockfile_sources(
        project_root,
        crate::supply_chain::RegistryOrigin {
            url: registry_url,
            ecosystem: "web",
        },
    )
    .map_err(|error| LockfileError::Mutation(error.to_string()))?;
    let registry_source = sources
        .iter()
        .find(|source| same_registry(source.url.as_str(), registry_url))
        .map(|source| source.id.clone())
        .unwrap_or_else(|| mgc_lockfile::v4_graph::source_id_for(registry_url.trim()));
    let mut lane = mgc_lockfile::v4_graph::build_v4_from_resolved_graph_with_source(
        graph,
        registry_url,
        &registry_source,
        mgc_lockfile::EcosystemTag::Web,
        owner_core,
        &generator,
    )
    .map_err(|error| LockfileError::Graph(error.to_string()))?;
    // The signed lock records the sources the project actually declared
    // (`mgc.toml [[sources]]`), each with its real claims/trust — not the
    // builder's single chain registry dressed as a wildcard (SEC-LOOP-04).
    // (Lock đã ký ghi đúng source project khai báo, kèm claim/trust thật.)
    lane.sources = sources;
    let base_material = base.clone();
    let (mut doc, dropped) = mgc_lockfile::v4_graph::merge_v4_lane_packages(
        base,
        lane.packages,
        lane.sources,
        lane.root_dependencies,
        owner_core,
        &generator,
    );
    if had_legacy_rows || dropped {
        eprintln!(
            "WARNING: non-v4 mgc.lock rows were replaced by the v4 lane write (sibling pins need `mgc migrate lock --to v4` or re-install under v4 schema)"
        );
    }
    // Signed v4 on disk is never rewritten without explicit re-signing.
    // (v4 đã ký trên đĩa không bao giờ bị ghi đè nếu không ký lại tường minh.)
    if let Some(base_doc) = &base_material {
        mgc_lockfile::atomic::ensure_v4_unsigned_for_rewrite(base_doc)
            .map_err(|error| LockfileError::Mutation(error.to_string()))?;
    }
    if let Some(base_doc) = &base_material
        && mgc_lockfile::v4_graph::v4_docs_match_material(base_doc, &doc)
        && !dropped
    {
        return Ok(());
    }
    doc.metadata.generated_at = chrono::Utc::now().to_rfc3339();
    let text = mgc_lockfile::writer::finalize_v4_document(&mut doc)
        .map_err(|error| LockfileError::Serialization(error.to_string()))?;
    mgc_lockfile::atomic::atomic_write_v4_bytes(lock_path, text.as_bytes())
        .map_err(|error| LockfileError::AtomicWrite(error.to_string()))?;
    Ok(())
}

/// Compare two registry base URLs ignoring a trailing slash.
/// So sánh hai base URL registry, bỏ qua slash cuối.
fn same_registry(declared: &str, configured: &str) -> bool {
    declared.trim().trim_end_matches('/') == configured.trim().trim_end_matches('/')
}
