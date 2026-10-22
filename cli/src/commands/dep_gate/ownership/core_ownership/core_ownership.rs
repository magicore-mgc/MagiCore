//! Explicit all-core policy dispatch — dispatch policy tường minh cho mọi core.
//! Unknown cores fail closed; every known core has one policy owner.
//! Core chưa biết fail-closed; mỗi core đã biết có một owner policy.

use super::{ai, app, cicd, cloud, game, hardware, iot, lib, web};
use crate::commands::dep_gate::{DepContext, DepOp, DepOwner};

/// Select exactly one core policy after shared evidence guards pass.
/// Chọn đúng một policy core sau khi guard bằng chứng chung đã qua.
pub(in crate::commands::dep_gate) fn owner_for_core(ctx: &DepContext<'_>, op: DepOp) -> DepOwner {
    match ctx.core {
        "web" => web::owner_for(ctx, op),
        "lib" => lib::owner_for(ctx, op),
        "ai" => ai::owner_for(ctx, op),
        "app" => app::owner_for(ctx, op),
        "game" => game::owner_for(ctx, op),
        "iot" => iot::owner_for(ctx, op),
        "clo" => cloud::owner_for(ctx, op),
        "hardware" => hardware::owner_for(ctx, op),
        "cicd" => cicd::owner_for(ctx, op),
        _ => DepOwner::Unsupported,
    }
}
