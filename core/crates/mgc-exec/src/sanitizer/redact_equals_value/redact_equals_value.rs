use crate::sanitizer::{REDACTED, is_sensitive_flag};

/// Return the established replacement for a sensitive `flag=value` item.
pub(crate) fn redact_equals_value(arg: &str) -> Option<String> {
    let (flag, _) = arg.split_once('=')?;
    if is_sensitive_flag(flag) {
        Some(format!("{flag}={REDACTED}"))
    } else {
        // Preserve the old branch behavior: every equals-form argument is
        // handled here, including values that resemble another sensitive flag.
        Some(arg.to_string())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
