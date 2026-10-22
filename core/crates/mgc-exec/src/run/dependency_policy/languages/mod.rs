//! Per-ecosystem dependency-resolution rules.
//! Rule resolve dependency riêng theo từng ecosystem.

#[path = "cargo/mod.rs"]
mod cargo;
#[path = "dotnet/mod.rs"]
mod dotnet;
#[path = "flutter/mod.rs"]
mod flutter;
#[path = "go/mod.rs"]
mod go;
#[path = "java/mod.rs"]
mod java;
#[path = "python/mod.rs"]
mod python;
#[path = "rustc/mod.rs"]
mod rustc;
#[path = "swift/mod.rs"]
mod swift;
#[path = "zig/mod.rs"]
mod zig;

#[cfg(test)]
pub(in crate::run) use go::is_go_toolchain_version_probe;

pub(super) fn denial_reason(
    tool: &str,
    args: &[String],
    first: &str,
    goproxy_disabled: bool,
) -> Option<&'static str> {
    match tool {
        "cargo" => cargo::denial_reason(args, first),
        "rustc" => rustc::denial_reason(args),
        "go" => go::denial_reason(args, first, goproxy_disabled),
        "python" | "py" => python::denial_reason(tool, args),
        "dotnet" => dotnet::denial_reason(args, first),
        "flutter" | "dart" => flutter::denial_reason(tool, args, first),
        "swift" => swift::denial_reason(args, first),
        "gradle" | "gradlew" | "mvn" | "mvnw" => java::denial_reason(tool, args),
        "zig" => zig::denial_reason(args),
        _ => None,
    }
}
