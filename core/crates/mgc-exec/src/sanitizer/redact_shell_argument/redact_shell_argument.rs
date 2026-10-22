use crate::sanitizer::{REDACTED, contains_embedded_sensitive_flag};

/// Redact a whole shell-like argv item when it contains a sensitive option.
pub(crate) fn redact_shell_argument(arg: &str) -> Option<String> {
    (arg.chars().any(char::is_whitespace) && contains_embedded_sensitive_flag(arg))
        .then(|| REDACTED.to_string())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
