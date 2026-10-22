//! Gate embedded-Web Cloud dependency projects — gate dependency Cloud dùng Web nhúng.
//! Requires a recognized CDK/Pulumi project with its root package.json.
//! Yêu cầu project CDK/Pulumi nhận diện được và package.json tại root.

use crate::commands::dep_gate::{DepContext, DepOp, eco, from_dep_flag, gate};
use anyhow::Result;

/// Gate cloud dependency operations only when the detected CDK/Pulumi
/// project actually has a JavaScript package manifest handled by the
/// embedded MGC Web engine. Terraform/other cloud types and manifest-less
/// projects remain Unsupported; this never opts into a provider PM.
/// (Chỉ mở lane CDK/Pulumi có package.json do engine Web nội bộ xử lý.)
pub fn gate_cloud_project(
    root: &std::path::Path,
    framework: &str,
    op: DepOp,
    compat_flag: Option<&str>,
) -> Result<()> {
    let ecosystem = (matches!(framework, "cdk" | "pulumi") && root.join("package.json").is_file())
        .then_some(eco::JS);
    let compat = from_dep_flag(compat_flag)?;
    gate(
        &DepContext::new("clo", ecosystem, Some(framework), None, op)
            .with_manifest_format(ecosystem.map(|_| "package-json")),
        None,
        &compat,
        Some(&root.join(".magicore").join("exec.log")),
    )
}
