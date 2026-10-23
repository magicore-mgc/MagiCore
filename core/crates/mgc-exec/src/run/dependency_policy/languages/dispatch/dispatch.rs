//! Ecosystem denial-reason router.
//! Router lý do từ chối theo ecosystem.

pub(in crate::run) fn denial_reason(
    tool: &str,
    args: &[String],
    first: &str,
    goproxy_disabled: bool,
) -> Option<&'static str> {
    match tool {
        "cargo" => super::super::cargo::denial_reason(args, first),
        "rustc" => super::super::rustc::denial_reason(args),
        "go" => super::super::go::denial_reason(args, first, goproxy_disabled),
        "python" | "py" => super::super::python::denial_reason(tool, args),
        "dotnet" => super::super::dotnet::denial_reason(args, first),
        "flutter" | "dart" => super::super::flutter::denial_reason(tool, args, first),
        "swift" => super::super::swift::denial_reason(args, first),
        "gradle" | "gradlew" | "mvn" | "mvnw" => super::super::java::denial_reason(tool, args),
        "zig" => super::super::zig::denial_reason(args),
        _ => None,
    }
}
