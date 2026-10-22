//! Normalize a script token to its process-manager identity.
//! Chuẩn hóa script token thành identity của process manager.

use super::super::super::super::process_basename;

pub(in crate::run::implementation) fn process_invocation_program(candidate: &str) -> String {
    let normalized = crate::allowlist::normalize_script_token(candidate)
        .unwrap_or_else(|| process_basename(candidate));
    normalized
        .strip_suffix(".bash")
        .or_else(|| normalized.strip_suffix(".sh"))
        .unwrap_or(&normalized)
        .to_string()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
