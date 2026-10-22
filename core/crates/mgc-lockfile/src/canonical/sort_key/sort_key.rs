//! Computes deterministic tie-break keys for packages and dependency edges.
//! Tạo khóa phụ tất định để sắp xếp package và cạnh phụ thuộc.

use crate::canonical::{PackageV4, edge_table::push_edge};
use crate::v4::Edge;
/// Sort key for packages: (ecosystem, name, version, source_id,
/// variant-debug) — Khóa sắp xếp package.
pub(in crate::canonical) fn package_sort_key(
    package: &PackageV4,
) -> (String, String, String, String, String) {
    // Feature identity is a set; sorting keys must match serialized normalization.
    // Danh tính feature là tập; khóa sort phải khớp chuẩn hóa khi serialize.
    let mut variant = package.key.variant.clone();
    if let Some(features) = &mut variant.feature_set {
        features.sort();
    }
    (
        package.key.ecosystem.as_str().to_owned(),
        package.key.name.clone(),
        package.key.version.clone(),
        package.key.source_id.clone(),
        format!("{variant:?}"),
    )
}

/// Sort key for edges — Khóa sắp xếp cạnh.
pub(in crate::canonical) fn edge_sort_key(
    edge: &Edge,
) -> (String, String, String, String, String, String) {
    (
        edge.target_key.ecosystem.as_str().to_owned(),
        edge.target_key.name.clone(),
        edge.target_key.version.clone(),
        edge.range.clone(),
        format!("{:?}", edge.kind),
        {
            // Break ties with every authenticated edge field, not input order.
            // Phân xử bằng mọi field đã ký, không phụ thuộc thứ tự đầu vào.
            let mut rendered = String::new();
            push_edge(&mut rendered, edge);
            rendered
        },
    )
}
