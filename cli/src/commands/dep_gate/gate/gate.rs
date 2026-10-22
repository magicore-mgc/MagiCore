//! Enforce one dependency ownership decision — cưỡng chế một quyết định ownership.
//! Refuses compatibility before checking exact native/scaffold/unsupported ownership.
//! Từ chối compat trước khi kiểm tra ownership native/scaffold/unsupported.

use crate::commands::compat::CompatMode;
use crate::commands::dep_gate::{DepContext, DepGateError, DepOwner, eco, owner_for};
use anyhow::{Error, Result};

/// The single control path: MUST be called by every dependency lane
/// before spawning a toolchain or routing to an adapter, with the lane's
/// full `DepContext` (core + detected ecosystem + framework + target +
/// operation). There is deliberately NO language-unaware shorthand —
/// callers that cannot determine the ecosystem fail Unsupported.
/// - `tool` and `audit_log` remain in the signature for source compatibility;
///   neither can authorize an external package manager.
///
/// `tool` và `audit_log` giữ chữ ký tương thích; không tham số nào mở PM ngoài.
///
/// Đường điều khiển duy nhất: mọi lane dependency PHẢI gọi trước khi
/// spawn toolchain hoặc gọi adapter, kèm `DepContext` đầy đủ. Cố ý KHÔNG
/// có dạng gọn bỏ-ngôn-ngữ — caller không xác định được ecosystem thì
/// fail Unsupported.
pub fn gate(
    ctx: &DepContext,
    _tool: Option<&str>,
    compat: &CompatMode,
    _audit_log: Option<&std::path::Path>,
) -> Result<()> {
    if let CompatMode::Explicit(value) = compat {
        return Err(DepGateError::compat_disabled(value).into());
    }
    match owner_for(ctx) {
        // Explicit compat was rejected above; these successful states do not spawn a provider.
        // Compat tường minh đã bị từ chối bên trên; hai trạng thái này không spawn provider.
        DepOwner::Native | DepOwner::ScaffoldOnly => Ok(()),
        DepOwner::Unsupported => {
            let detail = match (ctx.core, ctx.ecosystem, ctx.manifest_format) {
                ("lib", Some(eco::JAVA), Some("gradle")) => Some(
                    "Gradle build scripts are not dependency manifests; native Java dependency operations require a root pom.xml.",
                ),
                ("lib", Some(eco::JAVA), Some("ambiguous")) => Some(
                    "both Maven and Gradle manifests are present; select one build system before native Java dependency operations.",
                ),
                ("lib", Some(eco::JAVA), _) => Some(
                    "native Java dependency operations require a root pom.xml; Gradle is unsupported.",
                ),
                ("lib", Some(eco::DOTNET), _) => Some(
                    "native .NET dependency operations require exactly one root-level .csproj; multi-project selection is not implemented.",
                ),
                ("lib", Some(eco::PYTHON), Some("pyproject-unsupported")) => Some(
                    "native Python dependency operations require a plain PEP 621 project without foreign lockfiles, dependency groups, or alternate package-manager sources.",
                ),
                _ => None,
            };
            let error: Error =
                DepGateError::unsupported(ctx.core, ctx.op.as_str(), ctx.ecosystem).into();
            match detail {
                Some(detail) => Err(error.context(format!("{}: {detail}", ctx.describe()))),
                None => Err(error),
            }
        }
    }
}
