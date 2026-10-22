//! Game dependency lifecycle policy — policy lifecycle dependency Game.
//! Only Bevy/Cargo install, add, update, and remove have native owners.
//! Chỉ Bevy/Cargo install, add, update, remove có owner native.

use crate::commands::dep_gate::{DepContext, DepOp, DepOwner, eco};

pub(in crate::commands::dep_gate::ownership::core_ownership) fn owner_for(
    ctx: &DepContext<'_>,
    op: DepOp,
) -> DepOwner {
    match (ctx.ecosystem, op) {
        (Some(eco::BEVY), DepOp::Install | DepOp::Add | DepOp::Update | DepOp::Remove) => {
            DepOwner::Native
        }
        _ => DepOwner::Unsupported,
    }
}
