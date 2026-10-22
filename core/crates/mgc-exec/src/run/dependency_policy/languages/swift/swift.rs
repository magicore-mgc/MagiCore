//! SwiftPM dependency and automatic-resolution rules.
//! Rule dependency và automatic resolution của SwiftPM.

pub(in crate::run::implementation::dependency_policy::implementation::languages) fn denial_reason(
    args: &[String],
    first: &str,
) -> Option<&'static str> {
    let has = |flag: &str| args.iter().any(|arg| arg == flag);
    if has("package") {
        return Some("SwiftPM dependency-management commands are not allowed through mgc-exec");
    }
    if matches!(first, "build" | "test" | "run")
        && !(has("--skip-update") && has("--disable-automatic-resolution"))
    {
        return Some(
            "Swift compile/test commands must disable package updates and automatic resolution",
        );
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
