//! Go module mutation, compiler environment and built-in tool probe rules.
//! Rule mutation Go module, môi trường compile và probe tool tích hợp.

const GO_TOOLCHAIN_VERSION_PROBE_TOOLS: &[&str] = &[
    "asm",
    "cgo",
    "compile",
    "cover",
    "link",
    "preprofile",
    "vet",
];

pub(in crate::run) fn denial_reason(
    args: &[String],
    first: &str,
    goproxy_disabled: bool,
) -> Option<&'static str> {
    if is_go_toolchain_version_probe(args) {
        return None;
    }
    let has = |flag: &str| args.iter().any(|arg| arg == flag);
    match first {
        "mod" | "work" => Some("Go module/workspace commands are not allowed through mgc-exec"),
        "get" | "install" => {
            Some("Go dependency-management subcommands are not allowed through mgc-exec")
        }
        "list" => Some("Go list may resolve modules and is not allowed through mgc-exec"),
        "build" | "test" | "run" if !has("-mod=readonly") || !goproxy_disabled => {
            Some("Go compile/test commands must use -mod=readonly and GOPROXY=off")
        }
        _ => None,
    }
}

pub(in crate::run) fn is_go_toolchain_version_probe(args: &[String]) -> bool {
    matches!(
        args,
        [command, tool, version]
            if command == "tool"
                && GO_TOOLCHAIN_VERSION_PROBE_TOOLS.contains(&tool.as_str())
                && version == "-v=full"
    )
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
