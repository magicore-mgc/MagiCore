//! Hardware dependency operation policy — policy operation dependency Hardware.
//! Only the real read-only inventory operation is scaffold-only.
//! Chỉ lệnh inventory read-only thật được phân loại scaffold-only.

use crate::commands::dep_gate::{DepContext, DepOp, DepOwner};

pub(in crate::commands::dep_gate::ownership::core_ownership) fn owner_for(
    _ctx: &DepContext<'_>,
    op: DepOp,
) -> DepOwner {
    match op {
        DepOp::List => DepOwner::ScaffoldOnly,
        _ => DepOwner::Unsupported,
    }
}
