//! Shared test context builders — tạo context dùng chung cho test.
//! Mirrors the existing fixture helpers without production dependencies.
//! Giữ helper fixture hiện tại và không thêm dependency production.

use crate::commands::compat::CompatMode;
use crate::commands::dep_gate::{DepContext, DepOp, eco};

pub(crate) fn native() -> CompatMode {
    CompatMode::Native
}

pub(crate) fn explicit(tool: &str) -> CompatMode {
    CompatMode::Explicit(tool.to_string())
}

pub(crate) fn ctx<'a>(core: &'a str, ecosystem: Option<&'a str>, op: DepOp) -> DepContext<'a> {
    let context = DepContext::new(core, ecosystem, None, None, op);
    if core != "lib" {
        return context;
    }
    let format = match ecosystem {
        Some(eco::TS) => Some("package-json"),
        Some(eco::RUST) => Some("cargo-toml"),
        Some(eco::PYTHON) => Some("pep621-native"),
        Some(eco::GO) => Some("go-mod"),
        Some(eco::JAVA) => Some("maven-pom"),
        Some(eco::DOTNET) => Some("csproj"),
        _ => None,
    };
    context.with_manifest_format(format)
}

pub(crate) fn lib_ctx(
    ecosystem: &'static str,
    manifest_format: Option<&'static str>,
    op: DepOp,
) -> DepContext<'static> {
    DepContext::new("lib", Some(ecosystem), None, None, op).with_manifest_format(manifest_format)
}
