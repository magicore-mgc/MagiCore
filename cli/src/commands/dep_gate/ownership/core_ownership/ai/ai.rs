//! AI dependency lifecycle policy — policy lifecycle dependency AI.
//! AI/Python is native only after a caller proves the MGC-owned PEP 621 manifest.
//! AI/Python chỉ native khi caller chứng minh manifest PEP 621 do MGC sở hữu.

use crate::commands::dep_gate::{DepContext, DepOp, DepOwner, eco};

pub(in crate::commands::dep_gate::ownership::core_ownership) fn owner_for(
    ctx: &DepContext<'_>,
    _op: DepOp,
) -> DepOwner {
    match ctx.ecosystem {
        Some(eco::PYTHON) => DepOwner::Native,
        _ => DepOwner::Unsupported,
    }
}
