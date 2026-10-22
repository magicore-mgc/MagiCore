//! Cargo dependency-management and locked/offline command rules.
//! Rule command Cargo quản lý dependency và build locked/offline.

pub(in crate::run::implementation::dependency_policy::implementation::languages) fn denial_reason(
    args: &[String],
    first: &str,
) -> Option<&'static str> {
    let has = |flag: &str| args.iter().any(|arg| arg == flag);
    match first {
        "add" | "remove" | "update" | "fetch" | "install" | "vendor" | "generate-lockfile" => {
            Some("Cargo dependency-management subcommands are not allowed through mgc-exec")
        }
        "package" | "publish" | "search" | "login" | "logout" | "owner" | "yank" | "tree"
        | "info" => Some("Cargo registry/package subcommands are not allowed through mgc-exec"),
        "build" | "test" | "check" | "run" | "clippy" | "metadata"
            if !(has("--locked") && has("--offline")) =>
        {
            Some("Cargo compile/test commands must include --locked --offline")
        }
        "build" | "test" | "check" | "run" | "clippy" | "metadata" => None,
        "fmt" | "clean" | "version" | "help" | "locate-project" | "read-manifest"
        | "verify-project" => None,
        "" => None,
        _ => Some("unclassified Cargo subcommands are blocked by mgc-exec policy"),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
