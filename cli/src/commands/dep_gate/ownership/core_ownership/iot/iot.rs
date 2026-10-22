//! IoT dependency lifecycle policy — policy lifecycle dependency IoT.
//! Only the ESP32 Rust/Cargo lifecycle operations are currently native.
//! Hiện chỉ operation lifecycle ESP32 Rust/Cargo là native.

use crate::commands::dep_gate::{DepContext, DepOp, DepOwner};

pub(in crate::commands::dep_gate::ownership::core_ownership) fn owner_for(
    ctx: &DepContext<'_>,
    op: DepOp,
) -> DepOwner {
    match (ctx.ecosystem, op) {
        (Some("esp32-rust"), DepOp::Install | DepOp::Add | DepOp::Update | DepOp::Remove) => {
            DepOwner::Native
        }
        _ => DepOwner::Unsupported,
    }
}
