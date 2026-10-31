//! Compares two v4 documents by content, ignoring set-like collection order.
//! So sánh hai tài liệu v4 theo nội dung, bỏ qua thứ tự collection dạng tập.

use crate::canonical::LockfileV4;

/// True when both documents carry the same content.
///
/// The serializer emits packages, edges and sources in a canonical order,
/// but the TOML reader re-sorts package items and expands maps in its own
/// order. Comparing the raw vectors would therefore reject a faithful round
/// trip purely on ordering. Every field is still compared; only the
/// identified collections are normalized before the comparison.
/// (Đúng khi hai tài liệu cùng nội dung. Serializer xuất theo thứ tự
/// canonical nhưng reader sắp lại item package nên so Vec thô sẽ từ chối
/// một round-trip trung thực chỉ vì thứ tự. Mọi field vẫn được so; chỉ các
/// collection có identity được chuẩn hóa trước khi so.)
pub fn same_v4_content(left: &LockfileV4, right: &LockfileV4) -> bool {
    if left.version != right.version
        || left.metadata != right.metadata
        || left.sources != right.sources
        || left.peer_contexts != right.peer_contexts
        || left.root_dependencies != right.root_dependencies
        || left.root_dependencies_by_owner != right.root_dependencies_by_owner
        || left.optimizer_profile != right.optimizer_profile
        || normalize_workspace(left) != normalize_workspace(right)
    {
        return false;
    }
    normalize_packages(&left.payload()) == normalize_packages(&right.payload())
}

/// Canonical rendering of the workspace topology: `members` is a set, so it
/// is sorted before comparison; cross-core edges keep their declared order.
/// Biểu diễn canonical topology workspace: `members` là tập nên được sắp
/// trước khi so; cạnh liên core giữ thứ tự khai báo.
fn normalize_workspace(doc: &LockfileV4) -> Option<(Vec<String>, String)> {
    doc.workspace.as_ref().map(|workspace| {
        let mut members = workspace.members.clone();
        members.sort();
        let edges = workspace
            .cross_core_edges
            .iter()
            .map(|edge| format!("{}>{}:{}", edge.from_core, edge.to_core, edge.package))
            .collect::<Vec<_>>()
            .join(",");
        (members, edges)
    })
}

/// Canonical, order-independent rendering of the package table.
/// Biểu diễn canonical, không phụ thuộc thứ tự, của bảng package.
fn normalize_packages(payload: &crate::canonical::LockfilePayload) -> Vec<String> {
    let mut rows: Vec<String> = payload
        .packages
        .iter()
        .map(|package| {
            let mut edges: Vec<String> = package
                .edges
                .iter()
                .map(|edge| format!("{edge:?}"))
                .collect();
            edges.sort();
            let mut markers = package.markers.clone().unwrap_or_default();
            markers.sort();
            format!(
                "{:?}|{}|{}|{}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{edges:?}",
                package.key.ecosystem.as_str(),
                package.key.name,
                package.key.version,
                package.key.source_id,
                package.key.variant,
                markers,
                package.artifact,
                package.provenance,
                package.toolchain,
                package.scripts_policy,
                package.store_ref,
            )
        })
        .collect();
    rows.sort();
    rows
}
