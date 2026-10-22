//! Selects the root pins owned by one core, retaining the legacy global-root fallback only when ownership data is absent.
//! Chọn root pin của một core; chỉ dùng root toàn cục cho lock cũ không có bảng ownership.

use crate::v4_graph::error::error::unusable_for_install;
/// Owner tables are authoritative, including an explicitly empty root list.
/// Bảng owner có quyền quyết định, kể cả danh sách root rỗng; chỉ lock legacy dùng root chung.
use crate::{LockfileResult, canonical::LockfileV4};
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
            unusable_for_install(format!(
                "v4 lock has no root ownership entry for core '{owner_core}'"
            ))
        })
}
