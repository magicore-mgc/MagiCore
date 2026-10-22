//! Dart pub and Flutter package-resolution rules.
//! Rule pub Dart và resolve package Flutter.

pub(in crate::run::implementation::dependency_policy::implementation::languages) fn denial_reason(
    tool: &str,
    args: &[String],
    first: &str,
) -> Option<&'static str> {
    let has = |flag: &str| args.iter().any(|arg| arg == flag);
    if has("pub") {
        return Some("Dart/Flutter package-manager commands are not allowed through mgc-exec");
    }
    if tool == "flutter" && matches!(first, "build" | "test" | "run") && !has("--no-pub") {
        return Some("Flutter build/test/run commands must include --no-pub");
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
