use crate::protocols::interval_range::interval_matches;
use mgc_types::Version;

/// Maven version-range subset: `*`, bare version = caret (soft requirement
/// approximation — Maven's "soft" pin means ≥ this within the stream),
/// `[x]` exact, `[a,b)` / `(a,b]` intervals, comma = AND.
/// Subset khoảng version Maven: `*`, version trần = caret (xấp xỉ
/// requirement mềm — pin "mềm" của Maven nghĩa là ≥ giá trị đó trong cùng
/// dòng), `[x]` chính xác, khoảng `[a,b)` / `(a,b]`, phẩy = AND.
pub(in crate::protocols::maven) fn maven_matches(range: &str, version: &Version) -> bool {
    let range = range.trim();
    if range.is_empty() || range == "*" {
        return true;
    }
    if range.starts_with('[') || range.starts_with('(') {
        return maven_interval(range, version);
    }
    range
        .split(',')
        .map(str::trim)
        .all(|part| maven_part(part, version))
}

fn maven_interval(range: &str, version: &Version) -> bool {
    interval_matches(range, version)
}

fn maven_part(part: &str, version: &Version) -> bool {
    if part == "*" {
        return true;
    }
    if let Some(p) = part.strip_prefix("[") {
        // Inside a comma-split bare range this is a nested exact pin.
        // (Trong khoảng trần tách bằng phẩy, đây là pin chính xác lồng nhau.)
        let p = p.trim_end_matches(']');
        return Version::parse(p).map(|t| version == &t).unwrap_or(false);
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
    // Bare version = caret (minimum-version stream semantics).
    // (Version trần = caret (ngữ nghĩa dòng minimum-version).)
    let Some(target) = Version::parse(part).ok() else {
        return false;
    };
    if version < &target {
        return false;
    }
    if target.major > 0 {
        version.major == target.major
    } else if target.minor > 0 {
        version.major == 0 && version.minor == target.minor
    } else {
        version.major == 0 && version.minor == 0 && version.patch == target.patch
    }
}
