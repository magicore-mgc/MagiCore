//! Builds the canonical v4 document, registry source, root table, artifact records and peer-context digest table from a resolved graph.
//! Dựng lock v4 canonical gồm registry source, root ownership, artifact và bảng digest peer từ graph đã resolve.

use super::lane_source::lane_source;
use crate::LockfileResult;
use crate::policy::check_install_sri;
use crate::root_pin::format_root_pin;
use mgc_types::ResolvedGraph;

/// Build a v4 lock document directly from a resolved graph — the install
/// write path that reaches v4 parity with v3 (no lossy v3 detour).
/// Every installed pin carries its registry source, installer SRI (when
/// enforceable) and peer digest; peer sets are recorded in the canonical
/// `peer_contexts` table so verify/replay recompute instead of trusting.
/// (Dựng lock v4 trực tiếp từ graph đã resolve — đường ghi install đạt
/// parity v4 với v3 (không vòng qua v3 hao hụt). Mọi pin ghi nguồn registry,
/// SRI installer (khi thực thi được) và digest peer; tập peer ghi vào bảng
/// `peer_contexts` canonical để verify tính lại.)
///
/// Honest limits (giới hạn trung thực): edge `range` is `"*"` (the resolved
/// graph, like v3, does not retain declared ranges — only re-resolution
/// knows them); per-dependency dev flags are not persisted (same as v3);
/// `content_hash` stays empty until a lane hashes fetched bytes (same as
/// the v3→v4 migration); `target` stays `None` (single-target locks).
pub fn build_v4_from_resolved_graph(
    graph: &ResolvedGraph,
    registry_url: &str,
    ecosystem: crate::EcosystemTag,
    owner_core: &str,
    generator: &str,
) -> LockfileResult<crate::canonical::LockfileV4> {
    // The lane's real registry — id/claim derived from the URL it really
    // talked to, never a fixed `src-native` wildcard (SEC-LOOP-04).
    // (Registry thật của lane — id/claim suy từ URL thật đã gọi, không phải
    // wildcard `src-native` cố định.)
    let source = lane_source(registry_url, ecosystem);
    build_v4_with_source(
        graph,
        registry_url,
        ecosystem,
        owner_core,
        generator,
        source,
    )
}

/// Same builder with the caller-chosen source id (the source that really
/// served the lane when the project declared several sources).
/// (Cùng builder nhưng id source do caller chọn — source thật đã phục vụ lane
/// khi project khai nhiều source.)
pub fn build_v4_from_resolved_graph_with_source(
    graph: &ResolvedGraph,
    registry_url: &str,
    source_id: &str,
    ecosystem: crate::EcosystemTag,
    owner_core: &str,
    generator: &str,
) -> LockfileResult<crate::canonical::LockfileV4> {
    let mut source = lane_source(registry_url, ecosystem);
    source.id = source_id.to_string();
    build_v4_with_source(
        graph,
        registry_url,
        ecosystem,
        owner_core,
        generator,
        source,
    )
}

fn build_v4_with_source(
    graph: &ResolvedGraph,
    registry_url: &str,
    ecosystem: crate::EcosystemTag,
    owner_core: &str,
    generator: &str,
    source: crate::v4::SourceRef,
) -> LockfileResult<crate::canonical::LockfileV4> {
    use crate::canonical::{LockfileV4, LockfileV4Metadata, PackageV4};
    use crate::schema::{ArtifactRef, Provenance, SOURCE_KIND_NATIVE_RESOLVE};
    use crate::v4::{
        Edge, EdgeKind, EdgeOrigin, PackageKey, VariantKey, canonical_name, canonical_version,
        peer_context_digest,
    };

    let source_id = source.id.clone();
    let sources = vec![source];

    // Canonical peer table first: every distinct peer set gets one digest.
    // (Bảng peer canonical trước: mỗi tập peer khác nhau một digest.)
    let mut peer_contexts: std::collections::BTreeMap<String, Vec<(String, String)>> =
        std::collections::BTreeMap::new();
    let mut peer_digest_of = |peers: &[mgc_types::PackageId]| -> Option<String> {
        if peers.is_empty() {
            return None;
        }
        let mut set: Vec<(String, String)> = peers
            .iter()
            .map(|id| {
                (
                    canonical_name(ecosystem, id.name_str()),
                    canonical_version(&id.version().to_string()),
                )
            })
            .collect();
        set.sort();
        set.dedup();
        let digest = peer_context_digest(&set);
        peer_contexts
            .entry(digest.clone())
            .or_insert_with(|| set.clone());
        Some(digest)
    };

    let key_of = |id: &mgc_types::PackageId, peer_digest: Option<String>| PackageKey {
        ecosystem,
        name: canonical_name(ecosystem, id.name_str()),
        version: canonical_version(&id.version().to_string()),
        source_id: source_id.clone(),
        variant: VariantKey {
            peer_context: peer_digest,
            feature_set: None,
            target: None,
        },
    };

    let mut packages_v4 = Vec::with_capacity(graph.packages.len());
    let mut root_dependencies = Vec::new();
    for package in &graph.packages {
        let peer_digest = peer_digest_of(&package.peer_deps);
        let key = key_of(&package.id, peer_digest);
        if package.direct {
            root_dependencies.push(format!("{}@{}", key.name, key.version));
        }
        let mut edges = Vec::new();
        for dep in &package.deps {
            let target = key_of(dep, None);
            edges.push(Edge {
                target_key: target,
                range: "*".to_string(),
                kind: EdgeKind::Normal,
                origin: EdgeOrigin::Transitive {
                    from_key: Box::new(key.clone()),
                },
                marker: None,
            });
        }
        for peer in &package.peer_deps {
            let target = key_of(peer, None);
            edges.push(Edge {
                target_key: target,
                range: "*".to_string(),
                kind: EdgeKind::Peer,
                origin: EdgeOrigin::Transitive {
                    from_key: Box::new(key.clone()),
                },
                marker: None,
            });
        }
        // Installer SRI only when enforceable; content bytes are hashed by
        // the fetching lane, never fabricated here.
        // (Chỉ SRI installer thực thi được; byte content do lane tải hash,
        // không bịa ở đây.)
        let integrity_sri = (!package.integrity.is_empty())
            .then(|| check_install_sri(&package.integrity).ok())
            .flatten()
            .map(|_| package.integrity.clone());
        let artifact = if package.tarball_url.is_empty() {
            None
        } else {
            Some(ArtifactRef {
                url: package.tarball_url.clone(),
                size_bytes: None,
                content_hash: String::new(),
                downloaded_from: registry_url.to_string(),
                integrity_sri,
            })
        };
        packages_v4.push(PackageV4 {
            key,
            edges,
            markers: None,
            artifact,
            provenance: Some(Provenance {
                source_kind: SOURCE_KIND_NATIVE_RESOLVE.to_string(),
                tool: None,
                imported_from: None,
            }),
            toolchain: None,
            scripts_policy: None,
            store_ref: None,
        });
    }
    root_dependencies.sort();
    root_dependencies.dedup();

    let mut by_owner = std::collections::BTreeMap::new();
    by_owner.insert(
        owner_core.to_string(),
        root_dependencies
            .iter()
            .map(|pin| format_root_pin(ecosystem, pin))
            .collect(),
    );

    Ok(LockfileV4 {
        version: crate::v4::LOCKFILE_SCHEMA_V4.to_string(),
        metadata: LockfileV4Metadata {
            generated_at: String::new(),
            generator: generator.to_string(),
            lockfile_hash: String::new(),
            signature: None,
        },
        sources,
        peer_contexts,
        root_dependencies,
        root_dependencies_by_owner: by_owner,
        packages: packages_v4,
        workspace: None,
        optimizer_profile: None,
    })
}
