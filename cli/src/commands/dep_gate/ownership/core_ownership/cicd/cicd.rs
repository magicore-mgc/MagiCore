//! CI/CD dependency operation policy — policy dependency CI/CD.
//! CI/CD owns pipeline/scaffold operations, not package lifecycle operations.
//! CI/CD quản lý pipeline/scaffold, không quản lý package lifecycle.

use crate::commands::dep_gate::{DepContext, DepOp, DepOwner};

pub(in crate::commands::dep_gate::ownership::core_ownership) fn owner_for(
    _ctx: &DepContext<'_>,
    _op: DepOp,
) -> DepOwner {
    DepOwner::Unsupported
}
