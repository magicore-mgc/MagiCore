//! Split npm `name@version` identifiers — tách định danh npm `name@version`.
//! Scoped names split at the final `@`, preserving the scope prefix.
//! Tên có scope được tách tại `@` cuối để giữ nguyên phần scope.

/// Split `name@version` (name may be @scope/x — split on the LAST '@').
/// Tách `name@version` (tên có thể là @scope/x — tách tại '@' CUỐI).
pub(in crate::scanners::bun_deno) fn split_name_version(pin: &str) -> Option<(String, String)> {
    let at = pin.rfind('@')?;
    if at == 0 {
        return None;
    }
    let name = pin[..at].to_string();
    let version = pin[at + 1..].to_string();
    if name.is_empty() || version.is_empty() {
        return None;
    }
    Some((name, version))
}

#[cfg(test)]
#[path = "test/package_spec.rs"]
mod tests;
