//! `lockfile.rs` — public Web lock writer coordinator and v3 compatibility path.
//! điều phối writer lock Web công khai và đường tương thích v3

use std::path::Path;

use super::{
    LockfileError, atomic_write_web_lockfile, compute_tarball_integrity_reader, fresh_v3_lockfile,
    project_cache_dir, read_v4_document_if_present, read_web_lockfile_document,
    resolved_graphs_match_material, scope_web_lock_packages, web_lock_owner_core,
    web_lock_schema_is_v4, web_lockfile_matches_graph, write_web_lockfile_v4,
};
use mgc_lockfile::{LOCKFILE_SCHEMA_VERSION, Package};
use mgc_store::{Layout, PackageCache};
use mgc_types::{MgError, MgResult, adapter::ResolvedGraph};

pub fn write_web_lockfile_with_state(
    project_root: &Path,
    graph: &ResolvedGraph,
    registry_url: &str,
) -> MgResult<()> {
    let lock_path = project_root.join("mgc.lock");
    let owner_core = web_lock_owner_core(project_root)?;
    // Schema write target (Spec: easy config): `mgc.toml [lock] schema`
    // chooses `"4"`; anything else (including absent) keeps the v3 path
    // byte-identical to before. Unknown values fail closed here, never
    // silently fall back.
    // (Đích schema ghi (Spec: cấu hình dễ): `mgc.toml [lock] schema` chọn
    // `"4"`; còn lại giữ nguyên đường v3. Giá trị lạ fail-closed tại đây.)
    if web_lock_schema_is_v4(project_root)? {
        return write_web_lockfile_v4(project_root, &lock_path, graph, registry_url, &owner_core);
    }
    // v4 on disk: a replayed graph must round-trip byte-identically
    // instead of being rewritten (rebuilding v4 from ResolvedGraph would
    // drop variant/source identity — forbidden flattening). A diverged
    // graph falls back to a fresh v3 write with a LOUD downgrade warning
    // (same shape as the missing-lock default below, never silent).
    // (Lock v4 trên đĩa: graph replay phải giữ nguyên byte; graph lệch
    // thì ghi mới v3 kèm cảnh báo DOWNGRADE rõ ràng, không bao giờ lặng.)
    let v4_on_disk = read_v4_document_if_present(project_root)?;
    let had_v4_on_disk = v4_on_disk.is_some();
    if let Some(doc) = &v4_on_disk {
        let replayed = mgc_lockfile::v4_graph::graph_from_v4_lockfile(doc, &owner_core)
            .map_err(|error| LockfileError::Graph(error.to_string()))?;
        if resolved_graphs_match_material(&replayed, graph) {
            return Ok(());
        }
        eprintln!(
            "WARNING: installed graph diverged from the v4 mgc.lock — rewriting as v3 \
            (variant/source identity of old v4 pins is not preserved; re-run \
            `mgc migrate lock --to v4` for a lossless v4)"
        );
    }
    // A diverged v4 must NOT go through the legacy reader (it cannot
    // parse v4); it takes the same fresh-v3 default as a missing lock.
    // (v4 lệch không qua reader legacy — đi default v3 mới như lock thiếu.)
    let mut lockfile = match v4_on_disk {
        Some(_) => fresh_v3_lockfile(),
        None => read_web_lockfile_document(project_root)?.unwrap_or_else(fresh_v3_lockfile),
    };

    let mut scoped_lock = lockfile.clone();
    scope_web_lock_packages(&mut scoped_lock, &owner_core);
    if !had_v4_on_disk && lock_path.is_file() && web_lockfile_matches_graph(&scoped_lock, graph) {
        return Ok(());
    }
    mgc_lockfile::ensure_lockfile_mutation_allowed(&lock_path)
        .map_err(|error| LockfileError::Mutation(error.to_string()))?;

    let local_layout = Layout::new(project_cache_dir(project_root));
    let cache = PackageCache::new(local_layout.cache_dir())
        .map_err(|e| MgError::Store(e.to_string()))
        .ok();

    // Update metadata
    lockfile.metadata.generated_at = chrono::Utc::now().to_rfc3339();
    lockfile.metadata.generator = format!("mgc/{}", env!("CARGO_PKG_VERSION"));

    // Update packages
    let next_web_packages: Vec<Package> = graph
        .packages
        .iter()
        .map(|pkg| {
            let integrity = if pkg.integrity.is_empty() {
                if let Some(ref cache) = cache {
                    if let Ok(Some(file)) = cache.open_tarball(&pkg.id) {
                        compute_tarball_integrity_reader(file).unwrap_or_default()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            } else {
                pkg.integrity.clone()
            };

            Package {
                owner_core: Some(owner_core.clone()),
                name: pkg.id.name_str().to_string(),
                version: pkg.id.version().to_string(),
                resolved: pkg.tarball_url.clone(),
                integrity,
                dependencies: {
                    let mut deps: Vec<String> = pkg.deps.iter().map(ToString::to_string).collect();
                    deps.sort();
                    deps
                },
                peers: (!pkg.peer_deps.is_empty())
                    .then(|| pkg.peer_deps.iter().map(ToString::to_string).collect()),
                ecosystem: mgc_lockfile::EcosystemTag::Web,
                // Web is the one MGC-native engine — every entry records its
                // registry source and CAS ref so the lock owns the graph.
                // (Web là engine mgc-native duy nhất — mỗi entry ghi registry
                // source và CAS ref để lock sở hữu graph.)
                registry: Some(format!("npm://{registry_url}")),
                ..Package::default()
            }
        })
        .collect();
    let legacy_v2 = lockfile.version == "2";
    lockfile.packages.retain(|package| {
        !(package.ecosystem == mgc_lockfile::EcosystemTag::Web
            && (package.owner_core.as_deref() == Some(owner_core.as_str())
                || (owner_core == "web" && package.owner_core.is_none()))
            || (legacy_v2 && package.ecosystem == mgc_lockfile::EcosystemTag::Other))
    });
    lockfile.packages.extend(next_web_packages);
    let mut direct_roots: Vec<String> = graph
        .packages
        .iter()
        .filter(|package| package.direct)
        .map(|package| package.id.to_string())
        .collect();
    direct_roots.sort();
    mgc_lockfile::update_owner_root_pins(
        &mut lockfile,
        &owner_core,
        mgc_lockfile::EcosystemTag::Web,
        direct_roots,
    );
    lockfile.version = LOCKFILE_SCHEMA_VERSION.to_string();

    // Deterministic self-hash: v3 must never ship with an empty
    // `lockfile_hash`. The digest covers the document with that line
    // blanked, so two resolves of the same graph produce byte-identical
    // files (P0: `generated_at` used to differ on every run).
    // (Tự-hash tất định: v3 không được ghi `lockfile_hash` rỗng. Digest phủ
    // tài liệu khi dòng đó trống nên hai lần resolve cùng graph cho byte y
    // hệt (P0: trước đây `generated_at` khác nhau mỗi lần chạy).)
    let lockfile = mgc_lockfile::writer::with_lockfile_hash(&lockfile)
        .map_err(|e| LockfileError::Serialization(format!("lockfile hash failed: {}", e)))?;

    // Write lockfile — CANONICAL TOML v2 (một format duy nhất cho mọi đường ghi;
    // reader vẫn đọc được JSON-flavoured cũ từ các bản trước)
    // (Write CANONICAL TOML v2 — single format across all writers)
    let toml_content = mgc_lockfile::writer::serialize_lockfile(&lockfile)
        .map_err(|e| LockfileError::Serialization(format!("TOML serialization failed: {}", e)))?;

    atomic_write_web_lockfile(&lock_path, toml_content.as_bytes())?;

    Ok(())
}
