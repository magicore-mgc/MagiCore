//! Composer version-constraint parsing and comparison.
//! Parse và so sánh ràng buộc version kiểu Composer.

use mgc_types::Version;
use std::cmp::Ordering;

/// A Composer version as Composer itself normalises it: a numeric triple
/// plus an optional stability tag (`3.0.0-RC1` → `3.0.0` + `RC1`).
/// Version Composer sau khi chuẩn hoá: bộ ba số cộng tag stability tuỳ chọn
/// (`3.0.0-RC1` → `3.0.0` + `RC1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::protocols::composer) struct ComposerVersion {
    pub(in crate::protocols::composer) major: u64,
    pub(in crate::protocols::composer) minor: u64,
    pub(in crate::protocols::composer) patch: u64,
    /// `None` for a stable release; otherwise the normalised pre-release
    /// tag (`rc1`, `beta`, `dev`, `alpha`).
    /// `None` là bản ổn định; ngược lại là tag pre-release đã chuẩn hoá.
    pub(in crate::protocols::composer) stability: Option<String>,
}

impl ComposerVersion {
    /// Build a stable release triple.
    /// Dựng bộ ba release ổn định.
    pub(in crate::protocols::composer) fn release(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            stability: None,
        }
    }

    /// Parse the Composer spellings Packagist publishes: `3.12.1`,
    /// `3.0.0-RC1`, `1.0.0-beta2`, `2.0.0-dev`, `1.0-patch1`.
    /// Parse các cách viết Composer mà Packagist công bố.
    pub(in crate::protocols::composer) fn parse(raw: &str) -> Option<Self> {
        let (head, tail) = split_tail(raw.trim().trim_start_matches(['v', 'V']));
        let mut numbers = [0u64; 3];
        let mut count = 0;
        for (index, part) in head.split('.').enumerate() {
            if index > 2 || part.is_empty() || !part.chars().all(|ch| ch.is_ascii_digit()) {
                return None;
            }
            numbers[index] = part.parse().ok()?;
            count = index + 1;
        }
        if count == 0 {
            return None;
        }
        // `1.0.0-patch1` and `1.0.0-beta2` carry a counter that does not
        // change precedence; `-RC1`/`-beta` keep their qualifier.
        // `-patch1`/`-beta2` mang số đếm không đổi precedence; `-RC1`/`-beta`
        // giữ nguyên qualifier.
        let stability = match tail.trim_start_matches('-') {
            "" => None,
            tag => {
                let qualifier: String = tag
                    .chars()
                    .take_while(|ch| ch.is_ascii_alphabetic())
                    .collect::<String>()
                    .to_ascii_lowercase();
                let counter = &tag[qualifier.len()..];
                if qualifier.is_empty() || !counter.chars().all(|ch| ch.is_ascii_digit()) {
                    return None;
                }
                match qualifier.as_str() {
                    // `dev`/`patch`/`stable` do not lower precedence below
                    // the final release of the same triple.
                    // `dev`/`patch`/`stable` không xếp dưới bản final cùng bộ ba.
                    "stable" | "patch" => None,
                    "dev" if counter.is_empty() => Some("dev".to_string()),
                    other => Some(format!("{other}{counter}")),
                }
            }
        };
        Some(Self {
            major: numbers[0],
            minor: numbers[1],
            patch: numbers[2],
            stability,
        })
    }

    /// Release triple used for interval bounds (Composer's `N.N.N.0`).
    /// Bộ ba release dùng cho biên khoảng (dạng `N.N.N.0` của Composer).
    fn triple(&self) -> (u64, u64, u64) {
        (self.major, self.minor, self.patch)
    }

    /// Composer precedence: triple first, then a stable tag above every
    /// pre-release of the same triple.
    /// Precedence Composer: bộ ba trước, bản ổn định trên mọi pre-release
    /// cùng bộ ba.
    fn precedence_cmp(&self, other: &Self) -> Ordering {
        self.triple()
            .cmp(&other.triple())
            .then_with(|| match (&self.stability, &other.stability) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(left), Some(right)) => left.cmp(right),
            })
    }

    /// Adopt a shared [`Version`] (root requirement checks).
    /// Nhận [`Version`] dùng chung (kiểm tra root requirement).
    pub(in crate::protocols::composer) fn from_shared(version: &Version) -> Self {
        Self {
            major: version.major,
            minor: version.minor,
            patch: version.patch,
            // Normalise the tag the same way `parse` does (`RC1` → `rc1`)
            // so a subject and its range bound compare by Composer rules.
            // Chuẩn hoá tag giống `parse` (`RC1` → `rc1`) để subject và biên
            // range so theo đúng quy tắc Composer.
            stability: version
                .pre
                .as_deref()
                .map(|pre| {
                    let qualifier: String = pre
                        .chars()
                        .take_while(|ch| ch.is_ascii_alphabetic())
                        .collect();
                    let counter = &pre[qualifier.len()..];
                    format!("{}{counter}", qualifier.to_ascii_lowercase())
                })
                .or_else(|| version.dev.is_some().then(|| "dev".to_string())),
        }
    }
}

/// Composer constraint matcher.
/// Supports `*` / empty, `||` or `|` unions, comma or whitespace
/// conjunctions, exact, `= == >= <= > < !=`, `^`, `~`, `x.y.*` wildcards
/// and stability flags (`@dev`, `@stable`). Composer counts `1.2.3` as
/// `1.2.3.0`, and excluded intervals (`!=`, `<`) also drop that version's
/// pre-releases the way Composer's `VersionParser` does.
/// Matcher ràng buộc Composer: hỗ trợ union, conjunction, exact, toán tử
/// so sánh, `^`, `~`, wildcard và cờ stability. Composer coi `1.2.3` là
/// `1.2.3.0`; khoảng loại trừ cũng loại pre-release của version đó.
pub(in crate::protocols::composer) fn composer_matches(range: &str, version: &Version) -> bool {
    composer_matches_composer(range, &ComposerVersion::from_shared(version))
}

/// Same matcher over the protocol's own Composer version type.
/// Cùng matcher nhưng trên kiểu version Composer riêng của protocol.
pub(in crate::protocols::composer) fn composer_matches_composer(
    range: &str,
    version: &ComposerVersion,
) -> bool {
    let range = strip_stability_flags(range);
    let range = range.trim();
    if range.is_empty() || range == "*" {
        return true;
    }
    range.split("||").any(|alternative| {
        let alternative = alternative.trim();
        if alternative.is_empty() || alternative == "*" {
            return true;
        }
        split_and_terms(alternative)
            .into_iter()
            .all(|term| term_matches(&term, version))
    })
}

/// Stability flags such as `@dev` narrow the candidate set, not the
/// version arithmetic; strip them before matching.
/// Cờ stability (`@dev`…) thu hẹp ứng viên chứ không đổi phép so version.
pub(in crate::protocols::composer) fn strip_stability_flags(range: &str) -> String {
    let separated = range.replace("||", " | ");
    separated
        .split('|')
        .map(|alternative| {
            alternative
                .split_whitespace()
                .map(|term| match term.split_once('@') {
                    Some((body, _flag)) if !body.is_empty() => body,
                    Some(_) => "*",
                    None => term,
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("||")
}

/// Split an AND group on spaces and commas, keeping operator/version pairs
/// (`>= 1.2`, `!= 2.0`) together.
/// Tách nhóm AND theo dấu cách/dấu phẩy nhưng giữ cặp toán tử + version.
fn split_and_terms(group: &str) -> Vec<String> {
    let mut terms: Vec<String> = Vec::new();
    let mut pending_operator: Option<String> = None;
    for token in group.split(|ch: char| ch.is_whitespace() || ch == ',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        match split_leading_operator(token) {
            Some((operator, "")) => pending_operator = Some(operator.to_string()),
            Some((operator, rest)) => terms.push(format!("{operator}{rest}")),
            None => match pending_operator.take() {
                Some(operator) => terms.push(format!("{operator}{token}")),
                None => terms.push(token.to_string()),
            },
        }
    }
    terms
}

/// Split a leading comparison operator from a term (`>=1.2` → (`>=`, `1.2`)).
/// Tách toán tử so sánh ở đầu term (`>=1.2` → (`>=`, `1.2`)).
fn split_leading_operator(token: &str) -> Option<(&str, &str)> {
    for operator in ["!=", ">=", "<=", "==", "=", ">", "<", "^", "~"] {
        if let Some(rest) = token.strip_prefix(operator) {
            return Some((operator, rest));
        }
    }
    None
}

fn term_matches(term: &str, version: &ComposerVersion) -> bool {
    let term = term.trim();
    if term.is_empty() || term == "*" {
        return true;
    }
    if let Some(rest) = term.strip_prefix("!=") {
        return excluded(rest, version) || !exact(rest, version);
    }
    if let Some(rest) = term.strip_prefix(">=") {
        return parse_loose(rest).is_some_and(|target| version.precedence_cmp(&target).is_ge());
    }
    if let Some(rest) = term.strip_prefix("<=") {
        return parse_loose(rest).is_some_and(|target| version.precedence_cmp(&target).is_le());
    }
    if let Some(rest) = term.strip_prefix('>') {
        return parse_loose(rest).is_some_and(|target| version.precedence_cmp(&target).is_gt());
    }
    if let Some(rest) = term.strip_prefix('<') {
        return parse_loose(rest)
            .is_some_and(|target| excluded(rest, version) || version.triple() < target.triple());
    }
    if let Some(rest) = term.strip_prefix("==").or_else(|| term.strip_prefix('=')) {
        return exact(rest, version);
    }
    if let Some(rest) = term.strip_prefix('^') {
        return caret(rest, version);
    }
    if let Some(rest) = term.strip_prefix('~') {
        return tilde(rest, version);
    }
    if term.contains('*')
        || term.ends_with(".x")
        || term.ends_with(".X")
        || term.contains(".x.")
        || term.contains(".X.")
    {
        return wildcard(term, version);
    }
    exact(term, version)
}

/// Composer's excluded interval: a pre-release of the boundary release sits
/// below the boundary but belongs to the interval being excluded.
/// Khoảng loại trừ Composer: pre-release của bản biên nằm dưới biên nhưng
/// thuộc khoảng bị loại.
fn excluded(raw: &str, version: &ComposerVersion) -> bool {
    parse_loose(raw).is_some_and(|target| {
        target.stability.is_none() && target.triple() == version.triple() && version != &target
    })
}

/// Composer pads short versions with zeroes: `1.0` is exactly `1.0.0`.
/// Composer bù số 0: `1.0` chính là `1.0.0`.
fn exact(raw: &str, version: &ComposerVersion) -> bool {
    parse_loose(raw).is_some_and(|target| version.precedence_cmp(&target) == Ordering::Equal)
}

/// `^1.2.3` allows `<2.0.0`; `^0.3` allows `<0.4.0` (Composer's left-most
/// non-zero rule).
/// `^1.2.3` cho phép `<2.0.0`; `^0.3` cho phép `<0.4.0` (quy tắc số khác 0
/// trái nhất của Composer).
fn caret(raw: &str, version: &ComposerVersion) -> bool {
    let lower = match parse_loose(raw) {
        Some(lower) => lower,
        None => return false,
    };
    if version.precedence_cmp(&lower).is_lt() {
        return false;
    }
    let upper = if lower.major > 0 {
        ComposerVersion::release(lower.major + 1, 0, 0)
    } else if lower.minor > 0 {
        ComposerVersion::release(0, lower.minor + 1, 0)
    } else {
        ComposerVersion::release(0, 0, lower.patch + 1)
    };
    version.triple() < upper.triple()
}

/// `~1.2.3` allows `<1.3.0`; `~1.2` allows `<2.0.0`.
/// `~1.2.3` cho phép `<1.3.0`; `~1.2` cho phép `<2.0.0`.
fn tilde(raw: &str, version: &ComposerVersion) -> bool {
    let Some(lower) = parse_loose(raw) else {
        return false;
    };
    if version.precedence_cmp(&lower).is_lt() {
        return false;
    }
    let upper = if component_count(raw) >= 3 {
        ComposerVersion::release(lower.major, lower.minor + 1, 0)
    } else {
        ComposerVersion::release(lower.major + 1, 0, 0)
    };
    version.triple() < upper.triple()
}

/// `1.*` allows any `1.x`, `1.2.*` allows `1.2.x`.
/// `1.*` cho phép mọi `1.x`, `1.2.*` cho phép `1.2.x`.
fn wildcard(term: &str, version: &ComposerVersion) -> bool {
    let cleaned = term.replace(".x.", ".*.").replace(".X.", ".*.");
    let cleaned = cleaned
        .strip_suffix(".x")
        .or_else(|| cleaned.strip_suffix(".X"))
        .map(|head| format!("{head}.*"))
        .unwrap_or(cleaned);
    let head = cleaned.trim_end_matches('*').trim_end_matches('.');
    let parts: Vec<&str> = head.split('.').filter(|part| !part.is_empty()).collect();
    match parts.as_slice() {
        [] => true,
        [major] => major
            .parse::<u64>()
            .is_ok_and(|major| version.major == major),
        [major, minor, ..] => {
            major
                .parse::<u64>()
                .is_ok_and(|major| version.major == major)
                && minor
                    .parse::<u64>()
                    .is_ok_and(|minor| version.minor == minor)
        }
    }
}

/// Operators compare with Composer's zero-padding: `>=1.2` means `>=1.2.0`.
/// Toán tử so sánh theo kiểu bù số 0 của Composer: `>=1.2` là `>=1.2.0`.
fn parse_loose(raw: &str) -> Option<ComposerVersion> {
    ComposerVersion::parse(&pad(raw)?)
}

/// Pad a bare version to three numeric components (`8.1` → `8.1.0`).
/// Bù một version trần đủ ba thành phần số (`8.1` → `8.1.0`).
fn pad(raw: &str) -> Option<String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }
    let (head, tail) = split_tail(raw.trim_start_matches(['v', 'V']));
    let components: Vec<&str> = head.split('.').collect();
    if components.len() > 3
        || components.iter().any(|component| {
            component.is_empty() || !component.chars().all(|ch| ch.is_ascii_digit())
        })
    {
        return None;
    }
    let mut padded = head.to_string();
    for _ in components.len()..3 {
        padded.push_str(".0");
    }
    Some(format!("{padded}{tail}"))
}

/// Split a version into its numeric head and pre-release/build tail.
/// Tách version thành phần đầu số và đuôi pre-release/build.
fn split_tail(raw: &str) -> (&str, &str) {
    match raw.find(['-', '+', ' ']) {
        Some(index) => (&raw[..index], &raw[index..]),
        None => (raw, ""),
    }
}

fn component_count(raw: &str) -> usize {
    let (head, _) = split_tail(raw.trim().trim_start_matches(['v', 'V']));
    if head.is_empty() {
        return 0;
    }
    head.split('.').count()
}

#[cfg(test)]
#[path = "test/version_constraints/version_constraints.rs"]
mod tests;
