//! NuGet minimum-version, floating, interval and comparator range semantics.
//! Ngữ nghĩa range minimum, floating, interval và comparator của NuGet.

use crate::protocols::interval_range::interval_matches;
use mgc_types::Version;

/// NuGet version-range subset: `*`/empty = any, bare = exact
/// (PackageReference minimum semantics), `x.*` float, `[a,b]`/`(a,b]`
/// intervals, comma = AND.
/// Subset khoảng version NuGet: `*`/rỗng = bất kỳ, trần = chính xác (ngữ
/// nghĩa minimum của PackageReference), `x.*` float, khoảng
/// `[a,b]`/`(a,b]`, phẩy = AND.
pub(in crate::protocols::nuget) fn nuget_matches(range: &str, version: &Version) -> bool {
    let range = range.trim();
    if range.is_empty() || range == "*" {
        return true;
    }
    if range.starts_with('[') || range.starts_with('(') {
        return nuget_interval(range, version);
    }
    if let Some(prefix) = range.strip_suffix(".*") {
        // Floating major/minor: "1.*" → same major.
        // (Float major/minor: "1.*" → cùng major.)
        let parts: Vec<&str> = prefix.split('.').collect();
        return match parts.len() {
            1 => version.major == parts[0].parse().unwrap_or(u64::MAX),
            2 => {
                version.major == parts[0].parse().unwrap_or(u64::MAX)
                    && version.minor == parts[1].parse().unwrap_or(u64::MAX)
            }
            _ => false,
        };
    }
    range
        .split(',')
        .all(|part| nuget_part(part.trim(), version))
}

fn nuget_interval(range: &str, version: &Version) -> bool {
    interval_matches(range, version)
}

fn nuget_part(part: &str, version: &Version) -> bool {
    if part == "*" {
        return true;
    }
    if let Some(p) = part.strip_prefix(">=") {
        return Version::parse(p).map(|t| version >= &t).unwrap_or(false);
    }
    if let Some(p) = part.strip_prefix("<=") {
        return Version::parse(p).map(|t| version <= &t).unwrap_or(false);
    }
    if let Some(p) = part.strip_prefix('>') {
        return Version::parse(p).map(|t| version > &t).unwrap_or(false);
    }
    if let Some(p) = part.strip_prefix('<') {
        return Version::parse(p).map(|t| version < &t).unwrap_or(false);
    }
    // Bare = exact minimum pin (PackageReference semantics).
    // (Trần = ghim minimum chính xác (ngữ nghĩa PackageReference).)
    Version::parse(part).map(|t| version == &t).unwrap_or(false)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
