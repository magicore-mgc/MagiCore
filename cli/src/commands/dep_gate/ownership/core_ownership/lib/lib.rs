//! Library dependency lifecycle policy — policy lifecycle dependency Lib.
//! Concrete manifest formats are guarded before this operation table runs.
//! Guard manifest cụ thể chạy trước bảng operation này.

use crate::commands::dep_gate::{DepContext, DepOp, DepOwner, eco};

pub(in crate::commands::dep_gate::ownership::core_ownership) fn owner_for(
    ctx: &DepContext<'_>,
    op: DepOp,
) -> DepOwner {
    match (ctx.ecosystem, op) {
        (Some(eco::TS), _) => DepOwner::Native,
        (
            Some(eco::RUST | eco::PYTHON | eco::GO | eco::DOTNET | eco::JAVA),
            DepOp::Remove | DepOp::Update,
        ) => DepOwner::Native,
        (Some(eco::DOTNET), DepOp::Add) => DepOwner::Native,
        (Some(eco::JAVA), DepOp::Add) => DepOwner::Native,
        (Some(eco::PYTHON), DepOp::List) => DepOwner::Native,
        (Some(eco::RUST | eco::GO | eco::JAVA | eco::DOTNET), DepOp::List) => DepOwner::Unsupported,
        (Some(eco::RUST | eco::PYTHON | eco::GO | eco::JAVA | eco::DOTNET), _) => DepOwner::Native,
        _ => DepOwner::Unsupported,
    }
}
