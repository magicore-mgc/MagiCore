//! Go module version parsing and supported range operators.
//! Parse version module Go và các toán tử range được hỗ trợ.

use mgc_types::{MgError, MgResult, Version};

/// Go version-range subset: `*`, bare version = EXACT (go.mod pins are
/// minimum-version-selected already — the pin is the lock), plus `^`, `~`,
/// `=`, `>=`, `<=`, `>`, `<` (comma = AND).
/// Subset khoảng version Go: `*`, version trần = CHÍNH XÁC (pin go.mod đã
/// qua minimum-version-selection — pin chính là lock), cộng `^`, `~`, `=`,
/// `>=`, `<=`, `>`, `<` (phẩy = AND).
pub(in crate::protocols::go) fn go_matches(range: &str, version: &Version) -> bool {
    let range = range.trim().trim_start_matches('v');
    if range.is_empty() || range == "*" {
        return true;
    }
    range
        .split(',')
        .map(str::trim)
        .all(|part| go_part(part, version))
}

fn go_part(part: &str, version: &Version) -> bool {
    if part == "*" {
        return true;
    }
    if let Some(p) = part.strip_prefix(">=") {
        return parse_go_version(p).map(|t| version >= &t).unwrap_or(false);
    }
    if let Some(p) = part.strip_prefix("<=") {
        return parse_go_version(p).map(|t| version <= &t).unwrap_or(false);
    }
    if let Some(p) = part.strip_prefix('>') {
        return parse_go_version(p).map(|t| version > &t).unwrap_or(false);
    }
    if let Some(p) = part.strip_prefix('<') {
        return parse_go_version(p).map(|t| version < &t).unwrap_or(false);
    }
    if let Some(p) = part.strip_prefix('^') {
        return go_caret(p, version);
    }
    if let Some(p) = part.strip_prefix('~') {
        return go_tilde(p, version);
    }
    if let Some(p) = part.strip_prefix('=') {
        return parse_go_version(p).map(|t| version == &t).unwrap_or(false);
    }
    // Bare version = exact pin (go.mod semantics).
    // (Version trần = ghim chính xác (ngữ nghĩa go.mod).)
    parse_go_version(part)
        .map(|t| version == &t)
        .unwrap_or(false)
}

pub(in crate::protocols::go) fn parse_go_version(input: &str) -> MgResult<Version> {
    let raw = input.trim().trim_start_matches('v');
    let normalized = raw.strip_suffix("+incompatible").unwrap_or(raw);
    if normalized == raw && raw.contains('+') {
        return Err(MgError::InvalidVersion(input.to_string()));
    }
    Version::parse(normalized)
}

fn go_caret(raw: &str, version: &Version) -> bool {
    let Ok(target) = parse_go_version(raw.trim()) else {
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

fn go_tilde(raw: &str, version: &Version) -> bool {
    let Ok(target) = parse_go_version(raw.trim()) else {
        return false;
    };
    version >= &target && version.major == target.major && version.minor == target.minor
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
