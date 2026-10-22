//! Shared fail-closed ownership guards — guard ownership fail-closed dùng chung.
//! Validates manifest, GC, and offline evidence before projecting pipeline stages.
//! Kiểm chứng manifest, GC và offline trước khi quy operation pipeline về install.

use crate::commands::dep_gate::{DepContext, DepOp, eco};

/// Apply cross-core evidence guards before consulting any per-core policy.
/// Chạy guard bằng chứng dùng chung trước policy riêng của từng core.
pub(in crate::commands::dep_gate) fn preflight_allows(ctx: &DepContext<'_>) -> bool {
    if ctx.core == "ai"
        && ctx.ecosystem == Some(eco::PYTHON)
        && ctx.manifest_format != Some("mgc-pyproject")
    {
        return false;
    }
    if ctx.core == "lib" {
        let required_format = match ctx.ecosystem {
            Some(eco::TS) => Some("package-json"),
            Some(eco::RUST) => Some("cargo-toml"),
            Some(eco::PYTHON) => Some("pep621-native"),
            Some(eco::GO) => Some("go-mod"),
            Some(eco::JAVA) => Some("maven-pom"),
            Some(eco::DOTNET) => Some("csproj"),
            _ => None,
        };
        if required_format.is_some() && ctx.manifest_format != required_format {
            return false;
        }
    }
    if matches!(
        (ctx.core, ctx.ecosystem),
        ("game", Some(eco::BEVY)) | ("iot", Some("esp32-rust"))
    ) && ctx.manifest_format != Some("cargo-toml")
    {
        return false;
    }
    if ctx.op == DepOp::Gc
        && !(ctx.core == "web"
            && matches!(
                ctx.ecosystem,
                Some(eco::JS | eco::TS | "javascript" | "typescript")
            ))
    {
        return false;
    }
    if ctx.op == DepOp::OfflineReinstall && !offline_supported(ctx) {
        return false;
    }
    true
}

/// Project offline support from the exact lanes backed by the MGC store.
/// Suy ra offline từ đúng lane có cache MGC làm backend.
fn offline_supported(ctx: &DepContext<'_>) -> bool {
    matches!(
        (ctx.core, ctx.ecosystem, ctx.framework, ctx.manifest_format,),
        (
            "web",
            Some(eco::JS | eco::TS | "javascript" | "typescript"),
            _,
            _,
        ) | ("lib", Some(eco::TS), _, _)
            | ("lib", Some(eco::PYTHON), _, _)
            | ("ai", Some(eco::PYTHON), _, Some("mgc-pyproject"))
            | ("lib", Some(eco::RUST), _, Some("cargo-toml"))
            | ("app", Some(eco::FLUTTER), _, _)
            // CDK/Pulumi embeds WebAdapter and reuses its verified lock/CAS path.
            // CDK/Pulumi dùng WebAdapter và lock/CAS đã xác minh.
            | (
                "clo",
                Some(eco::JS | eco::TS | "javascript" | "typescript"),
                Some("cdk" | "pulumi"),
                Some("package-json")
            )
    )
}

/// Pipeline stages inherit only the matching install policy cell.
/// Stage pipeline chỉ kế thừa đúng cell install tương ứng.
pub(in crate::commands::dep_gate) fn install_projection(op: DepOp) -> DepOp {
    match op {
        DepOp::Resolve
        | DepOp::Lock
        | DepOp::Fetch
        | DepOp::Verify
        | DepOp::Store
        | DepOp::Materialize
        | DepOp::FrozenInstall => DepOp::Install,
        verb => verb,
    }
}
