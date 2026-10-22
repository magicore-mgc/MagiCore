//! App dependency lifecycle policy — policy lifecycle dependency App.
//! Each supported app ecosystem keeps its operation verbs explicit.
//! Mỗi ecosystem App nêu rõ operation được hỗ trợ.

use crate::commands::dep_gate::{DepContext, DepOp, DepOwner, eco};

pub(in crate::commands::dep_gate::ownership::core_ownership) fn owner_for(
    ctx: &DepContext<'_>,
    op: DepOp,
) -> DepOwner {
    match (ctx.ecosystem, op) {
        (Some(eco::RN), _) => DepOwner::Unsupported,
        (
            Some(eco::FLUTTER),
            DepOp::Install | DepOp::Add | DepOp::Update | DepOp::OfflineReinstall,
        ) => DepOwner::Native,
        (Some(eco::FLUTTER), DepOp::Remove) => DepOwner::Native,
        (Some(eco::FLUTTER), _) => DepOwner::Unsupported,
        (Some(eco::SWIFT), DepOp::Add | DepOp::Remove | DepOp::Update) => DepOwner::Unsupported,
        (Some(eco::SWIFT), DepOp::Install) => DepOwner::Native,
        (Some(eco::SWIFT), _) => DepOwner::Unsupported,
        (Some(eco::KOTLIN | eco::OBJC), _) => DepOwner::Unsupported,
        _ => DepOwner::Unsupported,
    }
}
