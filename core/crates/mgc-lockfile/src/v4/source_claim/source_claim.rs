//! Rank source claims and match wildcard package names.
//! Xếp hạng claim source và khớp tên package wildcard.

/// Claim specificity rank (higher wins; catch-all `*` is rank 0 and only
/// ever a fallback) — Hạng đặc hiệu của claim (cao thắng; `*` hạng 0 và
/// chỉ là fallback).
/// - 4: exact name (no wildcard).
/// - 3: scoped prefix (`@corp/*`).
/// - 2: infix/prefix wildcard (`corp-*`, `foo-*-bar`).
/// - 1: suffix wildcard (`*-corp`).
/// - 0: catch-all `*`.
///
/// Multiple wildcards degrade to the LOWEST matching rank (conservative).
pub fn claim_specificity(pattern: &str) -> u8 {
    if pattern == "*" {
        return 0;
    }
    if !pattern.contains('*') {
        return 4;
    }
    let stars = pattern.matches('*').count();
    if stars > 1 {
        // Multiple wildcards degrade to the lowest rank (conservative).
        // (Nhiều wildcard hạ về hạng thấp nhất (thận trọng).)
        return 1;
    }
    let looks_scoped_prefix = pattern.starts_with('@') && pattern.ends_with("/*");
    let looks_suffix = pattern.starts_with("*-");
    if looks_scoped_prefix {
        3
    } else if looks_suffix {
        1
    } else {
        // Single infix/prefix wildcard (`corp-*`, `foo-*-bar`).
        2
    }
}

/// Match a package name against a claim pattern (`*` = any run).
/// Khớp tên package với pattern claim.
pub fn claim_matches(pattern: &str, name: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if !pattern.contains('*') {
        return pattern == name;
    }
    let parts: Vec<&str> = pattern.split('*').collect();
    let mut rest = name;
    if !pattern.starts_with('*') {
        let Some(tail) = rest.strip_prefix(parts[0]) else {
            return false;
        };
        rest = tail;
    }
    for part in &parts[1..parts.len() - 1] {
        let Some(idx) = rest.find(part) else {
            return false;
        };
        rest = &rest[idx + part.len()..];
    }
    if !pattern.ends_with('*') {
        return rest.ends_with(parts[parts.len() - 1]);
    }
    true
}

#[cfg(test)]
#[path = "test/source_claim.rs"]
mod tests;
