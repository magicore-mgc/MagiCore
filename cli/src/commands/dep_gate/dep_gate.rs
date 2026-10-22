//! Single ownership decision coordinator — coordinator quyết định ownership duy nhất.
//! Keeps evidence guards ahead of pipeline projection and all per-core rules.
//! Giữ guard bằng chứng trước phép quy đổi pipeline và policy từng core.

use super::ownership::{install_projection, owner_for_core, preflight_allows};
use super::{DepContext, DepOwner};

/// Resolve one full dependency context to its single lifecycle owner.
/// Chọn owner lifecycle duy nhất từ context dependency đầy đủ.
/// Static ownership policy — the single source used by runtime gates and capability output.
/// Bảng ownership tĩnh — nguồn duy nhất cho gate runtime và capability.
pub fn owner_for(ctx: &DepContext) -> DepOwner {
    if !preflight_allows(ctx) {
        return DepOwner::Unsupported;
    }
    owner_for_core(ctx, install_projection(ctx.op))
}
