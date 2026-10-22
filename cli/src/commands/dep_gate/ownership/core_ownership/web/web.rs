//! Web dependency lifecycle policy — policy lifecycle dependency Web.
//! Web JS/TS cells use the embedded native npm pipeline; other ecosystems stay closed.
//! Web JS/TS dùng pipeline npm native nhúng; ecosystem khác bị chặn.

use crate::commands::dep_gate::{DepContext, DepOp, DepOwner, eco};

pub(in crate::commands::dep_gate::ownership::core_ownership) fn owner_for(
    ctx: &DepContext<'_>,
    _op: DepOp,
) -> DepOwner {
    match ctx.ecosystem {
        Some(eco::JS | eco::TS | "javascript" | "typescript") => DepOwner::Native,
        _ => DepOwner::Unsupported,
    }
}
