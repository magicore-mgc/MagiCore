//! Dart constraint parsing and comparison helpers.
//! Các helper parse và so sánh ràng buộc version Dart.

use mgc_types::Version;

/// Dart constraint matcher: `any`, `^x.y.z`, bare = exact, space-separated
/// AND (`>=x.y.z <z`), `x.y.*` wildcard.
/// Matcher ràng buộc Dart: `any`, `^x.y.z`, trần = exact, AND cách nhau bởi
/// dấu cách (`>=x.y.z <z`), `x.y.*` wildcard.
pub(in crate::protocols::pubdev) fn dart_matches(range: &str, version: &Version) -> bool {
    let range = range.trim();
    if range.is_empty() || range == "any" {
        return true;
    }
    range
        .split_whitespace()
        .all(|part| dart_part(part, version))
}

fn dart_part(part: &str, version: &Version) -> bool {
    if part == "any" || part == "*" {
        return true;
    }
    if let Some(p) = part.strip_prefix('^') {
        return dart_caret(p, version);
    }
    if let Some(p) = part.strip_prefix(">=") {
        return dart_ge(p, version);
    }
    if let Some(p) = part.strip_prefix("<=") {
        return dart_le(p, version);
    }
    if let Some(p) = part.strip_prefix('>') {
        return dart_gt(p, version);
    }
    if let Some(p) = part.strip_prefix('<') {
        return dart_lt(p, version);
    }
    if part.contains('*') {
        return dart_wildcard(part, version);
    }
    // bare version → exact.
    Version::parse(part)
        .map(|target| version == &target)
        .unwrap_or(false)
}

fn dart_caret(raw: &str, version: &Version) -> bool {
    let Some(target) = Version::parse(raw.trim()).ok() else {
        return false;
    };
    version >= &target && version.major == target.major
}

fn dart_wildcard(part: &str, version: &Version) -> bool {
    let base = part.trim_end_matches('*').trim_end_matches('.');
    let parts: Vec<&str> = base.split('.').collect();
    if parts.is_empty() {
        return true;
    }
    let major: u64 = parts[0].parse().unwrap_or(u64::MAX);
    if version.major != major {
        return false;
    }
    if let Some(minor) = parts.get(1) {
        let minor: u64 = minor.parse().unwrap_or(u64::MAX);
        return version.minor == minor;
    }
    true
}

fn dart_ge(raw: &str, version: &Version) -> bool {
    Version::parse(raw.trim())
        .map(|t| version >= &t)
        .unwrap_or(false)
}
fn dart_le(raw: &str, version: &Version) -> bool {
    Version::parse(raw.trim())
        .map(|t| version <= &t)
        .unwrap_or(false)
}
fn dart_gt(raw: &str, version: &Version) -> bool {
    Version::parse(raw.trim())
        .map(|t| version > &t)
        .unwrap_or(false)
}
fn dart_lt(raw: &str, version: &Version) -> bool {
    Version::parse(raw.trim())
        .map(|t| version < &t)
        .unwrap_or(false)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
