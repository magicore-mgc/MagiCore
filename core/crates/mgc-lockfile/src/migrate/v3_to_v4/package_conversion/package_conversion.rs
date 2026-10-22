//! Projects v3 package pins and uniquely resolvable names into v4 records.
//! Chiếu pin package v3 và tên dependency phân giải duy nhất thành record v4.

use crate::{
    Package,
    canonical::PackageV4,
    v4::{Edge, EdgeKind, EdgeOrigin, PackageKey, VariantKey},
};

use super::super::{SourceCatalog, carry_install_sri, package_source_id};

/// Convert packages while keeping warnings in package and dependency order.
/// Chuyển package, giữ thứ tự warning theo package và dependency ban đầu.
pub(in crate::migrate::v3_to_v4) fn convert_packages(
    packages: &[Package],
    catalog: &SourceCatalog,
) -> (Vec<PackageV4>, Vec<String>) {
    let mut converted = Vec::new();
    let mut warnings = Vec::new();

    for package in packages {
        let key = PackageKey {
            ecosystem: package.ecosystem,
            name: crate::v4::canonical_name(package.ecosystem, &package.name),
            version: crate::v4::canonical_version(&package.version),
            source_id: package_source_id(package, catalog),
            variant: VariantKey {
                peer_context: None,
                feature_set: package.extras.clone(),
                target: None,
            },
        };
        let edges = convert_edges(package, packages, catalog, &key, &mut warnings);
        record_peer_warning(package, &mut warnings);
        if package.integrity.is_empty() {
            warnings.push(format!(
                "pin {}@{} has no integrity: v4 verify will fail on it until re-fetched",
                package.name, package.version
            ));
        }
        converted.push(PackageV4 {
            key,
            edges,
            markers: package.markers.clone(),
            artifact: carry_install_sri(package, &mut warnings),
            provenance: package.provenance.clone(),
            toolchain: package.toolchain.clone(),
            scripts_policy: package.scripts_policy.clone(),
            store_ref: package.store_ref.clone(),
        });
    }

    (converted, warnings)
}

/// Preserve only unique dependency identities; warn instead of guessing.
/// Chỉ giữ identity dependency duy nhất; cảnh báo thay vì đoán.
fn convert_edges(
    package: &Package,
    packages: &[Package],
    catalog: &SourceCatalog,
    key: &PackageKey,
    warnings: &mut Vec<String>,
) -> Vec<Edge> {
    let mut edges = Vec::new();
    for dependency in &package.dependencies {
        let (dep_name, dep_version) = dependency
            .rsplit_once('@')
            .filter(|(name, version)| !name.is_empty() && !version.is_empty())
            .map_or((dependency.as_str(), None), |(name, version)| {
                (name, Some(version))
            });
        let hits: Vec<&Package> = packages
            .iter()
            .filter(|candidate| {
                candidate.name == dep_name
                    && dep_version.is_none_or(|version| candidate.version == version)
            })
            .collect();
        match hits.as_slice() {
            [hit] => edges.push(Edge {
                target_key: PackageKey {
                    ecosystem: hit.ecosystem,
                    name: crate::v4::canonical_name(hit.ecosystem, &hit.name),
                    version: crate::v4::canonical_version(&hit.version),
                    source_id: package_source_id(hit, catalog),
                    variant: VariantKey::default(),
                },
                range: "*".to_string(),
                kind: EdgeKind::Normal,
                origin: EdgeOrigin::Transitive {
                    from_key: Box::new(key.clone()),
                },
                marker: None,
            }),
            [] => warnings.push(format!(
                "dropped edge {} -> {}: no locked package matches that identity",
                package.name, dependency
            )),
            _ => warnings.push(format!(
                "dropped edge {} -> {}: {} locked packages share that name (ambiguous without versions)",
                package.name,
                dep_name,
                hits.len()
            )),
        }
    }
    edges
}

/// Warn when legacy peer names lack versions needed for peer digests.
/// Cảnh báo peer legacy thiếu version cần để tính peer digest.
fn record_peer_warning(package: &Package, warnings: &mut Vec<String>) {
    if let Some(peers) = package.peers.as_ref()
        && !peers.is_empty()
    {
        warnings.push(format!(
            "dropped {} peer(s) of {}: v3 peers carry no versions, peer digests cannot be computed",
            peers.len(),
            package.name
        ));
    }
}
