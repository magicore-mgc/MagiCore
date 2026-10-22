//! `edge_target.rs` — exact and unambiguous legacy lock edge resolution.
//! phân giải cạnh lock chính xác và cạnh legacy không nhập nhằng

use mgc_lockfile::Package;
use mgc_types::PackageId;

/// Resolve a lock edge by exact package identity; old locks with name-only
/// edges are accepted only when that name has one unambiguous version.
/// (Phục hồi cạnh bằng identity chính xác; lock cũ chỉ có tên được nhận khi
/// tên đó có đúng một version, tránh nối nhầm khi có nhiều version.)
pub(crate) fn locked_edge_target(
    edge: &str,
    packages_by_id: &std::collections::HashMap<PackageId, &Package>,
    ids_by_name: &std::collections::HashMap<String, Vec<PackageId>>,
) -> Option<PackageId> {
    if let Ok(id) = PackageId::parse(edge) {
        return packages_by_id.contains_key(&id).then_some(id);
    }
    let candidates = ids_by_name.get(edge)?;
    (candidates.len() == 1).then(|| candidates[0].clone())
}
