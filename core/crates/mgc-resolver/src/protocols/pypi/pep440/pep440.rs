use mgc_types::Version;

/// PEP 440 subset matcher: `==`, `==x.y.*`, `>=`, `<=`, `>`, `<`, `!=`,
/// `~=` (compatible release), comma = AND, bare version = exact.
/// Matcher PEP 440 subset: `==`, `==x.y.*`, `>=`, `<=`, `>`, `<`, `!=`,
/// `~=` (compatible release), phẩy = AND, version trần = exact.
pub(in crate::protocols::pypi) fn pep440_matches(range: &str, version: &Version) -> bool {
    let range = range.trim();
    if range.is_empty() || range == "*" {
        return true;
    }
    range
        .split(',')
        .map(str::trim)
        .all(|part| pep440_part(part, version))
}

pub(in crate::protocols::pypi) fn pep440_part(part: &str, version: &Version) -> bool {
    let part = part.trim();
    if part == "*" {
        return true;
    }
    if let Some(p) = part.strip_prefix("==") {
        return pep_eq(p, version);
    }
    // Single `=` is mgc's own exact-pin spelling (the orchestrator builds
    // `=version` ranges from resolved ids) — treat as exact, never as
    // no-match. (`=` đơn là cách ghim exact của mgc.)
    if let Some(p) = part.strip_prefix('=') {
        return pep_eq(p, version);
    }
    if let Some(p) = part.strip_prefix("~=") {
        return pep_compat(p, version);
    }
    if let Some(p) = part.strip_prefix("!=") {
        return !pep_eq(p, version);
    }
    if let Some(p) = part.strip_prefix(">=") {
        return pep_ge(p, version);
    }
    if let Some(p) = part.strip_prefix("<=") {
        return pep_le(p, version);
    }
    if let Some(p) = part.strip_prefix('>') {
        return pep_gt(p, version);
    }
    if let Some(p) = part.strip_prefix('<') {
        return pep_lt(p, version);
    }
    // bare version → exact.
    pep_eq(part, version)
}

pub(in crate::protocols::pypi) fn pep_eq(raw: &str, version: &Version) -> bool {
    let raw = raw.trim();
    if let Some(base) = raw.strip_suffix(".*") {
        let base = base.trim_end_matches('.');
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
    } else {
        Version::parse(raw)
            .map(|target| version == &target)
            .unwrap_or(false)
    }
}

pub(in crate::protocols::pypi) fn pep_compat(raw: &str, version: &Version) -> bool {
    let raw = raw.trim();
    let Some(target) = Version::parse(raw).ok() else {
        return false;
    };
    if version < &target {
        return false;
    }
    let parts: Vec<&str> = raw.split('.').collect();
    if parts.len() <= 2 {
        // ~=1.4 → >=1.4,<2.0
        version.major == target.major
    } else {
        // ~=1.4.5 → >=1.4.5,<1.5
        version.major == target.major && version.minor == target.minor
    }
}

pub(in crate::protocols::pypi) fn pep_ge(raw: &str, version: &Version) -> bool {
    Version::parse(raw.trim())
        .map(|t| version >= &t)
        .unwrap_or(false)
}

pub(in crate::protocols::pypi) fn pep_le(raw: &str, version: &Version) -> bool {
    Version::parse(raw.trim())
        .map(|t| version <= &t)
        .unwrap_or(false)
}

pub(in crate::protocols::pypi) fn pep_gt(raw: &str, version: &Version) -> bool {
    Version::parse(raw.trim())
        .map(|t| version > &t)
        .unwrap_or(false)
}

pub(in crate::protocols::pypi) fn pep_lt(raw: &str, version: &Version) -> bool {
    Version::parse(raw.trim())
        .map(|t| version < &t)
        .unwrap_or(false)
}
