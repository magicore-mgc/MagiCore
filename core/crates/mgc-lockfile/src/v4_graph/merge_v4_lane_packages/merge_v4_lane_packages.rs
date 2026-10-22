//! Replaces one owner/ecosystem lane, preserves sibling rows, merges sources and roots, and prunes unreachable package and peer records.
//! Thay dữ liệu của một owner/ecosystem, giữ lane sibling, gộp source/root và dọn package/peer không còn reachable.

use std::collections::{HashMap, HashSet};

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
