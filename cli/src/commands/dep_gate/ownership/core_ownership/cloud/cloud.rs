//! Cloud dependency lifecycle policy — policy lifecycle dependency Cloud.
//! CDK/Pulumi package.json cells route through the embedded Web engine.
//! Cell CDK/Pulumi có package.json đi qua engine Web nhúng.

use crate::commands::dep_gate::{DepContext, DepOp, DepOwner, eco};

pub(in crate::commands::dep_gate::ownership::core_ownership) fn owner_for(
    ctx: &DepContext<'_>,
    _op: DepOp,
) -> DepOwner {
    match (ctx.ecosystem, ctx.framework, ctx.manifest_format) {
        (
            Some(eco::JS | eco::TS | "javascript" | "typescript"),
            Some("cdk" | "pulumi"),
            Some("package-json"),
        ) => DepOwner::Native,
        _ => DepOwner::Unsupported,
    }
}
