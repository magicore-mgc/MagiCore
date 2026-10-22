//! Checks explicit or manifest-derived roots, version ranges, selected ecosystems and orphan-free graph reachability before lock replay.
//! Kiểm root tường minh hoặc từ manifest, range version, ecosystem được chọn và graph không có package mồ côi trước khi replay lock.

use crate::canonical::LockfileV4;
use crate::root_pin::parse_root_pin;
use crate::v4::canonical_name;
use crate::v4_graph::v4_root_pins_for_owner::v4_root_pins_for_owner::v4_root_pins_for_owner;
use mgc_types::{Manifest, Version};
use std::collections::{HashMap, HashSet};

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
    // Chọn điểm neo từ root tường minh, nếu không thì mỗi dependency manifest phải khớp đúng một bản.
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
        // Mỗi range trong manifest phải được ít nhất một root thỏa mãn.
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
        // Manifest phải nhận diện mọi root được khai báo tường minh.
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
    // Duyệt BFS trên mọi cạnh; mọi package thuộc ecosystem được chọn phải
    // be reachable (orphan-free). Selected = ecosystems of the anchors.
    // tới được từ root đã chọn, không mồ côi; ecosystem được chọn lấy từ các anchor.
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
