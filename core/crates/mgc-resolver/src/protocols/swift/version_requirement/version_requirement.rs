use mgc_types::Version;

/// Swift range semantics: `*`/empty = any, `exact:X`/bare `X` = pin,
/// `from:X` = >=X same major, `minor:X` = >=X same major+minor,
/// `range:A..<B` = >=A && <B, `||` alternatives; branch/revision prefixes
/// never match a registry version (fail-closed at the caller).
/// Ngữ nghĩa range Swift: `*`/rỗng = bất kỳ, `exact:X`/`X` trần = pin,
/// `from:X` = >=X cùng major, `minor:X` = >=X cùng major+minor,
/// `range:A..<B` = >=A && <B, phương án `||`; tiền tố branch/revision
/// không bao giờ khớp version registry (fail-closed ở caller).
pub fn swift_matches(range: &str, version: &Version) -> bool {
    range.trim().split("||").any(|part| {
        let part = part.trim();
        if part.is_empty() || part == "*" {
            return true;
        }
        swift_part(part, version)
    })
}

fn swift_part(part: &str, version: &Version) -> bool {
    if let Some(v) = part.strip_prefix("exact:") {
        return Version::parse(v.trim())
            .map(|t| version == &t)
            .unwrap_or(false);
    }
    if let Some(v) = part.strip_prefix("from:") {
        return from_matches(v.trim(), version, false);
    }
    if let Some(v) = part.strip_prefix("minor:") {
        return from_matches(v.trim(), version, true);
    }
    if let Some(range) = part.strip_prefix("range:") {
        let Some((a, b)) = range.split_once("..<") else {
            return false;
        };
        let lower = Version::parse(a.trim());
        let upper = Version::parse(b.trim());
        return lower.map(|t| version >= &t).unwrap_or(false)
            && upper.map(|t| version < &t).unwrap_or(false);
    }
    if part.starts_with("branch:") || part.starts_with("revision:") || part.starts_with("tag:") {
        // Non-version requirements never select a registry version.
        // (Yêu cầu phi-version không bao giờ chọn version registry.)
        return false;
    }
    // Bare version = exact pin (Package.resolved semantics).
    // (Version trần = pin chính xác (ngữ nghĩa Package.resolved).)
    Version::parse(part.trim())
        .map(|t| version == &t)
        .unwrap_or(false)
}

fn from_matches(raw: &str, version: &Version, minor_only: bool) -> bool {
    let Ok(target) = Version::parse(raw) else {
        return false;
    };
    version >= &target
        && version.major == target.major
        && (!minor_only || version.minor == target.minor)
}
