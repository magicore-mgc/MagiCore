/// Sensitive keyword catalog used to classify long option names.
const SENSITIVE_PARTS: &[&str] = &[
    "token",
    "password",
    "pass",
    "_authtoken",
    "_password",
    "key",
    "secret",
    "apikey",
    "api_key",
    "auth",
];

/// Short options that are always treated as sensitive.
const SENSITIVE_SHORT_FLAGS: &[&str] = &["-p"];

/// True if `arg` is a sensitive flag. False positives are safer than leaks.
pub(crate) fn is_sensitive_flag(arg: &str) -> bool {
    let lower = arg.to_ascii_lowercase();
    let flag = lower.trim_start_matches('-');
    if flag.is_empty() {
        return false;
    }
    for part in SENSITIVE_PARTS {
        let needle = part.trim_start_matches('_');
        if flag
            .split(['-', '_'])
            .any(|segment| segment == needle || segment.starts_with(needle))
        {
            return true;
        }
    }
    SENSITIVE_SHORT_FLAGS.contains(&arg)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
