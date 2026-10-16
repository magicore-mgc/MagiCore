//! Lossless v4 lockfile → install-graph conversion + manifest closure.
//! Single home for both: CLI install paths and adapter resolve
//! short-circuits MUST call these instead of keeping local copies, so the
//! lossy-shape refusal rules cannot drift between copies.
//! (Chuyển lock v4 → graph install lossless + closure manifest — nhà
//! duy nhất: mọi đường install/resolve đều gọi vào đây, không copy riêng.)

use std::collections::{HashMap, HashSet};

use mgc_types::{Manifest, PackageId, ResolvedGraph, ResolvedPackage, Version};

use crate::canonical::LockfileV4;
use crate::policy::check_install_sri;
use crate::root_pin::{format_root_pin, parse_root_pin};
use crate::v4::{EdgeKind, canonical_name};
use crate::{LockfileError, LockfileResult};

fn bad(why: String) -> LockfileError {
    LockfileError::UnusableForInstall(why)
}

/// Owner tables are authoritative, including an explicitly empty root list.
/// Bảng owner có quyền quyết định, kể cả danh sách root rỗng; chỉ lock legacy dùng root chung.
pub fn v4_root_pins_for_owner<'a>(
    doc: &'a LockfileV4,
    owner_core: &str,
) -> LockfileResult<&'a [String]> {
    if doc.root_dependencies_by_owner.is_empty() {
        return Ok(&doc.root_dependencies);
    }
    doc.root_dependencies_by_owner
        .get(owner_core)
        .map(Vec::as_slice)
        .ok_or_else(|| {
            bad(format!(
                "v4 lock has no root ownership entry for core '{owner_core}'"
            ))
        })
}

/// Build an install graph from a v4 lock document WITHOUT flattening v4
/// identity — every lossy shape fails closed instead of guessing:
/// duplicate `name@version` instances (distinct variant/source would
/// collapse into one node), pins without installer-enforced SRI or
/// without an artifact URL (the materializer could never verify them),
/// edges naming no locked package, and unparseable ids all refuse.
/// Edge kinds map losslessly: Normal/Optional/Dev → `deps`,
/// Peer → `peer_deps`; `dev` = reachable from owner roots only through
/// Dev edges (roots themselves are never dev).
/// (Dựng graph install từ lock v4 KHÔNG ép identity — shape hao hụt nào
/// cũng fail-closed thay vì đoán.)
pub fn graph_from_v4_lockfile(doc: &LockfileV4, owner_core: &str) -> LockfileResult<ResolvedGraph> {
    // 1. Index by name@version; a second instance would collapse.
    let mut index: HashMap<String, usize> = HashMap::new();
    for (position, package) in doc.packages.iter().enumerate() {
        let id = format!("{}@{}", package.key.name, package.key.version);
        if index.insert(id.clone(), position).is_some() {
            return Err(bad(format!(
                "duplicate locked instance '{id}' (distinct variant/source cannot share one graph node)"
            )));
        }
    }

    // 2. Owner roots (qualified pins normalize to plain ids).
    let raw_roots = v4_root_pins_for_owner(doc, owner_core)?;
    let scoped_roots = !doc.root_dependencies_by_owner.is_empty() || !raw_roots.is_empty();
    let root_ids: HashSet<String> = raw_roots
        .iter()
        .map(|root| parse_root_pin(root).package_id.to_string())
        .collect();

    // 3. Adjacency for edge mapping + dev reachability.
    let mut adjacency: Vec<Vec<(usize, bool)>> = vec![Vec::new(); doc.packages.len()];
    for (position, package) in doc.packages.iter().enumerate() {
        for edge in &package.edges {
            let target_id = format!("{}@{}", edge.target_key.name, edge.target_key.version);
            let Some(&target) = index.get(&target_id) else {
                return Err(bad(format!(
                    "edge from '{}@{}' names no locked package ('{target_id}')",
                    package.key.name, package.key.version
                )));
            };
            adjacency[position].push((target, matches!(edge.kind, EdgeKind::Dev)));
        }
    }
    // Reachable through any edge vs reachable without Dev edges.
    let mut reachable = vec![false; doc.packages.len()];
    let mut prod_reachable = vec![false; doc.packages.len()];
    let mut stack = Vec::new();
    for id in &root_ids {
        stack.push(
            *index
                .get(id)
                .ok_or_else(|| bad(format!("owner root '{id}' names no locked package")))?,
        );
    }
    for &root in &stack {
        reachable[root] = true;
        prod_reachable[root] = true;
    }
    while let Some(current) = stack.pop() {
        for &(target, is_dev) in &adjacency[current] {
            if !reachable[target] {
                reachable[target] = true;
                stack.push(target);
            }
            if prod_reachable[current] && !is_dev && !prod_reachable[target] {
                prod_reachable[target] = true;
                stack.push(target);
            }
        }
    }

    if scoped_roots {
        // Validate union coverage before filtering: replay must not silently discard orphan pins.
        // Kiểm phủ hợp root trước khi lọc để replay không lặng bỏ pin mồ côi.
        let all_roots: Vec<&String> = if doc.root_dependencies_by_owner.is_empty() {
            raw_roots.iter().collect()
        } else {
            doc.root_dependencies_by_owner.values().flatten().collect()
        };
        let mut covered = vec![false; doc.packages.len()];
        let mut pending = Vec::new();
        for root in all_roots {
            let id = parse_root_pin(root).package_id;
            pending.push(
                *index
                    .get(id)
                    .ok_or_else(|| bad(format!("owner root '{root}' names no locked package")))?,
            );
        }
        while let Some(position) = pending.pop() {
            if covered[position] {
                continue;
            }
            covered[position] = true;
            pending.extend(adjacency[position].iter().map(|(target, _)| *target));
        }
        if let Some(position) = covered.iter().position(|covered| !covered) {
            let package = &doc.packages[position];
            return Err(bad(format!(
                "locked pin '{}@{}' is not reachable from any root ownership entry",
                package.key.name, package.key.version
            )));
        }
    }

    // 4. Packages.
    let mut packages = Vec::with_capacity(doc.packages.len());
    for (position, package) in doc.packages.iter().enumerate() {
        // Only this owner's closure is installed; sibling artifacts are not executable inputs.
        // Chỉ cài closure của owner đang chọn; artifact sibling không thuộc đầu vào thực thi.
        if scoped_roots && !reachable[position] {
            continue;
        }
        let id = format!("{}@{}", package.key.name, package.key.version);
        let package_id = PackageId::parse(&id).map_err(|error| {
            bad(format!(
                "locked instance '{id}' is not a valid package id: {error}"
            ))
        })?;
        let artifact = package.artifact.as_ref().ok_or_else(|| {
            bad(format!(
                "pin '{id}' has no artifact reference (no URL to fetch)"
            ))
        })?;
        if artifact.url.is_empty() {
            return Err(bad(format!("pin '{id}' has an empty artifact URL")));
        }
        let Some(sri) = artifact.integrity_sri.as_deref() else {
            return Err(bad(format!(
                "pin '{id}' has no installer-enforced SRI (re-resolve a lane that supplies registry SRI)"
            )));
        };
        if let Err(error) = check_install_sri(sri) {
            return Err(bad(format!("pin '{id}' carries {error}")));
        }
        let mut deps = Vec::new();
        let mut peer_deps = Vec::new();
        for edge in &package.edges {
            let target_id = format!("{}@{}", edge.target_key.name, edge.target_key.version);
            let target = PackageId::parse(&target_id).map_err(|error| {
                bad(format!(
                    "edge target '{target_id}' is not a valid package id: {error}"
                ))
            })?;
            match edge.kind {
                EdgeKind::Normal | EdgeKind::Optional | EdgeKind::Dev => deps.push(target),
                EdgeKind::Peer => peer_deps.push(target),
            }
        }
        let qualified_root = format_root_pin(package.key.ecosystem, &id);
        packages.push(ResolvedPackage {
            id: package_id,
            integrity: sri.to_string(),
            tarball_url: artifact.url.clone(),
            deps,
            peer_deps,
            direct: root_ids.contains(&id) || raw_roots.contains(&qualified_root),
            dev: reachable[position] && !prod_reachable[position],
        });
    }
    Ok(ResolvedGraph { packages })
}

/// v4 manifest-closure check. Two root modes:
/// - explicit roots (owner slice, else global list): roots must resolve,
///   every manifest range must match ≥1 same-name instance, every root
///   must be claimed by the manifest, and every locked package of the
///   manifest-selected ecosystems must be reachable from the roots;
/// - no roots (web locks deliberately carry none — manifest is the root
///   source of truth): each manifest dependency must match EXACTLY ONE
///   same-name instance, and every package of the manifest-selected
///   ecosystems must be reachable from those anchors.
///
/// (Kiểm tra closure v4: có roots thì roots phải thật + manifest khớp +
/// không mồ côi; không roots thì manifest làm neo với khớp duy-nhất.)
pub fn v4_graph_matches_manifest_closure(
    doc: &LockfileV4,
    manifest: &Manifest,
    owner_core: &str,
) -> bool {
    let Ok(raw_roots) = v4_root_pins_for_owner(doc, owner_core) else {
        return false;
    };
    let owner_scoped = !doc.root_dependencies_by_owner.is_empty();
    if owner_scoped && raw_roots.is_empty() {
        return manifest.all_dependencies().next().is_none();
    }
    if doc.packages.is_empty() {
        return raw_roots.is_empty() && manifest.all_dependencies().next().is_none();
    }
    let by_id: HashMap<String, usize> = doc
        .packages
        .iter()
        .enumerate()
        .map(|(position, package)| {
            (
                format!("{}@{}", package.key.name, package.key.version),
                position,
            )
        })
        .collect();
    let name_matches = |dependency_name: &str| -> Vec<usize> {
        doc.packages
            .iter()
            .enumerate()
            .filter(|(_, package)| {
                package.key.name == dependency_name
                    || package.key.name == canonical_name(package.key.ecosystem, dependency_name)
            })
            .map(|(position, _)| position)
            .collect()
    };
    // Seed anchors: explicit roots, else exactly-one manifest matches.
    let mut anchors = Vec::new();
    if raw_roots.is_empty() {
        for dependency in manifest.all_dependencies() {
            let candidates: Vec<usize> = name_matches(dependency.name.as_str())
                .into_iter()
                .filter(|position| {
                    Version::parse(&doc.packages[*position].key.version)
                        .is_ok_and(|version| dependency.range.matches(&version))
                })
                .collect();
            if candidates.len() != 1 {
                return false;
            }
            anchors.push(candidates[0]);
        }
    } else {
        for root in raw_roots {
            let id = parse_root_pin(root).package_id.to_string();
            let Some(&position) = by_id.get(&id) else {
                return false;
            };
            anchors.push(position);
        }
        // Every manifest range must be satisfied (any-match).
        let ranges_ok = manifest.all_dependencies().all(|dependency| {
            anchors.iter().any(|position| {
                let package = &doc.packages[*position];
                (package.key.name == dependency.name.as_str()
                    || package.key.name
                        == canonical_name(package.key.ecosystem, dependency.name.as_str()))
                    && Version::parse(&package.key.version)
                        .is_ok_and(|version| dependency.range.matches(&version))
            })
        });
        if !ranges_ok {
            return false;
        }
        // Every explicit root must be claimed by the manifest.
        let claimed = anchors.iter().all(|position| {
            let package = &doc.packages[*position];
            manifest.all_dependencies().any(|dependency| {
                (package.key.name == dependency.name.as_str()
                    || package.key.name
                        == canonical_name(package.key.ecosystem, dependency.name.as_str()))
                    && Version::parse(&package.key.version)
                        .is_ok_and(|version| dependency.range.matches(&version))
            })
        });
        if !claimed {
            return false;
        }
    }
    // BFS over all edges; every package of the selected ecosystems must
    // be reachable (orphan-free). Selected = ecosystems of the anchors.
    let mut reached = vec![false; doc.packages.len()];
    let mut stack = anchors.clone();
    while let Some(current) = stack.pop() {
        if reached[current] {
            continue;
        }
        reached[current] = true;
        for edge in &doc.packages[current].edges {
            let target_id = format!("{}@{}", edge.target_key.name, edge.target_key.version);
            let Some(&target) = by_id.get(&target_id) else {
                return false;
            };
            stack.push(target);
        }
    }
    let selected: HashSet<crate::EcosystemTag> = anchors
        .iter()
        .map(|position| doc.packages[*position].key.ecosystem)
        .collect();
    if owner_scoped {
        // Keep orphan detection across shared owners without treating valid siblings as orphans.
        // Kiểm orphan theo hợp các closure owner, tránh coi sibling hợp lệ là package mồ côi.
        let mut owned = vec![false; doc.packages.len()];
        let mut pending = Vec::new();
        for root in doc.root_dependencies_by_owner.values().flatten() {
            let Some(&position) = by_id.get(parse_root_pin(root).package_id) else {
                return false;
            };
            pending.push(position);
        }
        while let Some(position) = pending.pop() {
            if owned[position] {
                continue;
            }
            owned[position] = true;
            for edge in &doc.packages[position].edges {
                let id = format!("{}@{}", edge.target_key.name, edge.target_key.version);
                let Some(&target) = by_id.get(&id) else {
                    return false;
                };
                pending.push(target);
            }
        }
        return doc
            .packages
            .iter()
            .enumerate()
            .filter(|(_, package)| selected.contains(&package.key.ecosystem))
            .all(|(position, _)| owned[position]);
    }
    if selected.is_empty() {
        return reached.iter().all(|seen| *seen);
    }
    doc.packages
        .iter()
        .enumerate()
        .filter(|(_, package)| selected.contains(&package.key.ecosystem))
        .all(|(position, _)| reached[position])
}

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
    use crate::canonical::{LockfileV4, LockfileV4Metadata, PackageV4};
    use crate::schema::{ArtifactRef, Provenance, SOURCE_KIND_NATIVE_RESOLVE};
    use crate::v4::{
        Edge, EdgeKind, EdgeOrigin, PackageKey, SourceRef, VariantKey, canonical_name,
        canonical_version, peer_context_digest,
    };

    // One source entry for the lane registry that produced this graph.
    // (Một entry nguồn cho registry của lane đã cho ra graph này.)
    let source_id = "src-native".to_string();
    let sources = vec![SourceRef {
        id: source_id.clone(),
        url: registry_url.to_string(),
        ecosystem,
        priority: 100,
        claims: vec!["*".to_string()],
        trusted: false,
        allow_hosts: Vec::new(),
        allow_cidrs: Vec::new(),
        allow_protocols: vec!["https".to_string()],
    }];

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

/// Merge one lane's packages into a v4 document without touching sibling
/// lanes: entries owned by `(owner_core, ecosystem)` are replaced by the
/// lane build, everything else is preserved byte-for-byte. Sources merge by
/// id (lane source wins on conflict — same id means same registry URL by
/// construction).
/// (Gộp package một lane vào lock v4, không đụng lane khác: entry thuộc
/// `(owner_core, ecosystem)` được thay bằng bản build của lane, còn lại giữ
/// nguyên byte. Sources gộp theo id.)
///
/// Lane ownership of an entry is carried out-of-band (the caller knows
/// which `(owner, ecosystem)` it just resolved); v4 pins themselves do not
/// store owner strings — roots do, via `root_dependencies_by_owner`.
/// (Quyền lane của entry do caller biết (vừa resolve xong); pin v4 không lưu
/// owner — root lưu, qua `root_dependencies_by_owner`.)
///
/// Returns the merged document plus whether any sibling pin was dropped
/// because the on-disk base was not v4 (caller warns loudly, never silent).
/// (Trả document đã gộp và cờ có pin khác bị rớt vì base trên đĩa không
/// phải v4 (caller cảnh báo rõ, không lặng).)
pub fn merge_v4_lane_packages(
    base: Option<crate::canonical::LockfileV4>,
    lane_packages: Vec<crate::canonical::PackageV4>,
    lane_sources: Vec<crate::v4::SourceRef>,
    lane_roots: Vec<String>,
    owner_core: &str,
    generator: &str,
) -> (crate::canonical::LockfileV4, bool) {
    use crate::canonical::LockfileV4;
    use std::collections::{BTreeMap, HashSet};

    let mut dropped_sibling = false;
    // Lane package names decide replacement: same canonical (name, version,
    // source) pins are the lane's own rows.
    // (Tên package của lane quyết định thay thế.)
    let lane_ids: HashSet<(String, String, String)> = lane_packages
        .iter()
        .map(|p| {
            (
                p.key.name.clone(),
                p.key.version.clone(),
                p.key.source_id.clone(),
            )
        })
        .collect();

    let mut doc = base.unwrap_or_else(|| LockfileV4::new(generator));
    if !crate::v4::is_v4_schema_version(&doc.version) {
        dropped_sibling = true;
        doc = LockfileV4::new(generator);
    }
    // Any write through the current model uses the revision that authenticates markers.
    // Mọi lần ghi bằng model hiện tại dùng revision đã xác thực markers.
    doc.version = crate::v4::LOCKFILE_SCHEMA_V4.to_string();
    // Sibling rows survive only when the base was already v4; a legacy
    // base cannot contribute lossless rows.
    // (Chỉ base v4 mới giữ được row khác; base legacy không đóng góp lossless.)
    let kept: Vec<crate::canonical::PackageV4> = match base_has_v4(&doc, &lane_ids) {
        Some(rows) => rows,
        None => {
            dropped_sibling = true;
            Vec::new()
        }
    };
    // Roots first: lane roots replace this owner's entry (other owners
    // untouched). Callers without root tracking (legacy single-shot writers
    // that resolve the whole lane set) seed from the lane packages
    // themselves — every lane package is then live by definition.
    // (Root trước: root lane thay entry owner này (owner khác giữ nguyên).
    // Caller không theo dõi root (writer đơn cũ resolve cả tập lane) seed
    // từ package lane — mọi package lane khi đó mặc nhiên còn sống.)
    let mut by_owner = doc.root_dependencies_by_owner.clone();
    let mut roots = lane_roots;
    if roots.is_empty() {
        roots = lane_packages
            .iter()
            .map(|p| format!("{}@{}", p.key.name, p.key.version))
            .collect();
    }
    roots.sort();
    roots.dedup();
    by_owner.insert(owner_core.to_string(), roots);
    let mut flat: Vec<String> = by_owner.values().flatten().cloned().collect();
    flat.sort();
    flat.dedup();

    let mut packages = kept;
    packages.extend(lane_packages);

    // Sources merge by id; lane entries win (same id ⇒ same URL).
    // (Sources gộp theo id; entry lane thắng.)
    let mut sources: BTreeMap<String, crate::v4::SourceRef> = BTreeMap::new();
    for source in doc.sources.drain(..).chain(lane_sources) {
        sources.insert(source.id.clone(), source);
    }

    // Orphan collection AFTER roots are final: rows unreachable from any
    // current root are stale pins (e.g. a bumped version no lane claims
    // anymore), not sibling data — sibling lanes keep their roots above.
    // (Thu gom mồ côi SAU khi root chốt: row không tới được từ root hiện
    // hành nào là pin cũ, không phải dữ liệu khác — lane khác giữ root ở
    // trên nên row của họ vẫn tới được.)
    {
        use std::collections::HashSet;
        let mut reachable: HashSet<(String, String)> = HashSet::new();
        let mut stack: Vec<(String, String)> = Vec::new();
        for root in flat.iter().chain(by_owner.values().flatten()) {
            if let Some(pair) = split_root_name_version(root)
                && reachable.insert(pair.clone())
            {
                stack.push(pair);
            }
        }
        let mut by_id: HashMap<(String, String), Vec<usize>> = HashMap::new();
        for (position, package) in packages.iter().enumerate() {
            by_id
                .entry((package.key.name.clone(), package.key.version.clone()))
                .or_default()
                .push(position);
        }
        let mut kept_positions: HashSet<usize> = HashSet::new();
        while let Some(pair) = stack.pop() {
            if let Some(positions) = by_id.get(&pair) {
                for &position in positions {
                    if kept_positions.insert(position) {
                        for edge in &packages[position].edges {
                            stack.push((
                                edge.target_key.name.clone(),
                                edge.target_key.version.clone(),
                            ));
                        }
                    }
                }
            }
        }
        let mut survivors = Vec::with_capacity(kept_positions.len());
        for (position, package) in packages.into_iter().enumerate() {
            if kept_positions.contains(&position) {
                survivors.push(package);
            }
        }
        packages = survivors;
    }

    // Peer table is the union of surviving rows, rebuilt from edges so the
    // table stays verifiable (no stale digest lingers past the GC above).
    // (Bảng peer là hợp của row còn lại, dựng lại từ cạnh để luôn kiểm
    // chứng được (không sót digest cũ sau GC trên).)
    let mut peer_contexts: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for package in &packages {
        let Some(digest) = &package.key.variant.peer_context else {
            continue;
        };
        let mut peers: Vec<(String, String)> = package
            .edges
            .iter()
            .filter(|e| matches!(e.kind, crate::v4::EdgeKind::Peer))
            .map(|e| (e.target_key.name.clone(), e.target_key.version.clone()))
            .collect();
        peers.sort();
        peers.dedup();
        peer_contexts
            .entry(digest.clone())
            .or_default()
            .extend(peers);
    }
    for peers in peer_contexts.values_mut() {
        peers.sort();
        peers.dedup();
    }
    doc.sources = sources.into_values().collect();
    doc.peer_contexts = peer_contexts;
    doc.root_dependencies = flat;
    doc.root_dependencies_by_owner = by_owner;
    doc.packages = packages;
    doc.metadata.generator = generator.to_string();
    (doc, dropped_sibling)
}

/// Split a root pin (`mgc-root-v1:<eco>:<name@version>` or plain
/// `name@version`) into `(name, version)`. Scoped names (`@scope/name`)
/// split on the LAST `@`.
/// (Tách pin root thành `(name, version)`. Tên scope tách ở `@` cuối.)
fn split_root_name_version(pin: &str) -> Option<(String, String)> {
    let id = if let Some(encoded) = pin.strip_prefix("mgc-root-v1:") {
        encoded.split_once(':').map(|(_, id)| id).unwrap_or(encoded)
    } else {
        pin
    };
    let (name, version) = id.rsplit_once('@')?;
    if name.is_empty() || version.is_empty() {
        return None;
    }
    Some((name.to_string(), version.to_string()))
}

/// Keep rows that are NOT this lane's own (caller-filtered by identity);
/// `None` when the base document is unusable as a merge base.
/// (Giữ row KHÔNG phải của lane này; `None` khi base không dùng merge được.)
fn base_has_v4(
    doc: &crate::canonical::LockfileV4,
    lane_ids: &HashSet<(String, String, String)>,
) -> Option<Vec<crate::canonical::PackageV4>> {
    if !crate::v4::is_v4_schema_version(&doc.version) {
        return None;
    }
    Some(
        doc.packages
            .iter()
            .filter(|p| {
                !lane_ids.contains(&(
                    p.key.name.clone(),
                    p.key.version.clone(),
                    p.key.source_id.clone(),
                ))
            })
            .cloned()
            .collect(),
    )
}

/// Material equality between two v4 documents, ignoring volatile metadata
/// (timestamps, digests, signatures): same packages, sources, peer table
/// and roots means a rewrite would churn bytes for nothing.
/// (Bằng nhau material giữa hai lock v4, bỏ qua metadata dễ đổi
/// (timestamp, digest, chữ ký): cùng package, nguồn, bảng peer và root
/// nghĩa là ghi lại chỉ khuấy byte vô ích.)
pub fn v4_docs_match_material(
    first: &crate::canonical::LockfileV4,
    second: &crate::canonical::LockfileV4,
) -> bool {
    // Canonical ordering matches digest semantics without trusting a stored digest.
    // Chuẩn hóa thứ tự như digest; không tin giá trị hash được ghi trên đĩa.
    let mut first_payload = first.payload();
    let mut second_payload = second.payload();
    first_payload.generator.clear();
    second_payload.generator.clear();
    crate::canonical::canonical_toml(&first_payload)
        == crate::canonical::canonical_toml(&second_payload)
}
