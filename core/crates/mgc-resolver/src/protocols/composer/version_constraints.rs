//! Composer version-constraint parsing and comparison.
//! Parse và so sánh ràng buộc version kiểu Composer.

use mgc_types::Version;

/// Composer constraint matcher.
/// Supports `*` / empty, `||`/`|` union, `,`/space AND, exact,
/// `= == >= <= > < !=`, `^`, `~`, `x.y.*` wildcards and stability flags
/// (`@dev`, `@stable`). Composer counts `1.2.3` as `1.2.3.0`; our `*`
/// wildcard therefore requires the upper bound to stay inside the prefix.
/// Matcher ràng buộc Composer: hỗ trợ tổ hợp trên. Composer coi `1.2.3`
/// là `1.2.3.0` nên wildcard `*` phải chặn đúng phạm vi tiền tố.
pub(in crate::protocols::composer) fn composer_matches(range: &str, version: &Version) -> bool {
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
fn strip_stability_flags(range: &str) -> String {
    range
        .split('|')
        .map(|alternative| {
            alternative
                .split_whitespace()
                .map(|term| {
                    let (body, _) = match term.split_once('@') {
                        Some((body, _flag)) => (body, Some(())),
                        None => (term, None),
                    };
                    if body.is_empty() { "*" } else { body }
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
    for token in group.split([' ', ',']) {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        if matches!(token, ">=" | "<=" | ">" | "<" | "=" | "==" | "!=") {
            pending_operator = Some(token.to_string());
            continue;
        }
        match pending_operator.take() {
            Some(operator) => terms.push(format!("{operator}{token}")),
            None => terms.push(token.to_string()),
        }
    }
    terms
}

fn term_matches(term: &str, version: &Version) -> bool {
    let term = term.trim();
    if term.is_empty() || term == "*" {
        return true;
    }
    if let Some(rest) = term.strip_prefix("!=") {
        return parse_loose(rest.trim()).is_none_or(|target| version != &target);
    }
    if let Some(rest) = term.strip_prefix(">=") {
        return parse_loose(rest.trim()).is_some_and(|target| version >= &target);
    }
    if let Some(rest) = term.strip_prefix("<=") {
        return parse_loose(rest.trim()).is_some_and(|target| version <= &target);
    }
    if let Some(rest) = term.strip_prefix('>') {
        return parse_loose(rest.trim()).is_some_and(|target| version > &target);
    }
    if let Some(rest) = term.strip_prefix('<') {
        return parse_loose(rest.trim()).is_some_and(|target| version < &target);
    }
    if let Some(rest) = term
        .strip_prefix("==")
        .or_else(|| term.strip_prefix('='))
    {
        return exact(rest.trim(), version);
    }
    if let Some(rest) = term.strip_prefix('^') {
        return caret(rest.trim(), version);
    }
    if let Some(rest) = term.strip_prefix('~') {
        return tilde(rest.trim(), version);
    }
    if term.contains('*') || term.ends_with(".x") || term.contains(".x.") {
        return wildcard(term, version);
    }
    exact(term, version)
}

/// `1.0` matches `1.0.0`, not `1.0.5` (Composer pads with zeroes).
/// `1.0` khớp `1.0.0` chứ không khớp `1.0.5` (Composer bù số 0).
fn exact(raw: &str, version: &Version) -> bool {
    match component_count(raw) {
        0 | 1 => Version::parse(raw).is_ok_and(|target| version.major == target.major),
        2 => Version::parse(raw).is_ok_and(|target| {
            version.major == target.major && version.minor == target.minor && no_tail(version)
        }),
        _ => Version::parse(raw).is_ok_and(|target| version == &target),
    }
}

/// `^1.2.3` allows `<2.0.0`; `^0.3` allows `<0.4.0` (Composer's left-most
/// non-zero rule).
/// `^1.2.3` cho phép `<2.0.0`; `^0.3` cho phép `<0.4.0` (quy tắc số khác 0
/// trái nhất của Composer).
fn caret(raw: &str, version: &Version) -> bool {
    let Some(lower) = parse_loose(raw) else {
        return false;
    };
    if version < &lower {
        return false;
    }
    let upper = if lower.major > 0 {
        Version::new(lower.major + 1, 0, 0)
    } else if lower.minor > 0 || component_count(raw) >= 2 {
        Version::new(0, lower.minor + 1, 0)
    } else {
        Version::new(0, 0, lower.patch + 1)
    };
    version < &upper
}

/// `~1.2.3` allows `<1.3.0`; `~1.2` allows `<2.0.0`.
/// `~1.2.3` cho phép `<1.3.0`; `~1.2` cho phép `<2.0.0`.
fn tilde(raw: &str, version: &Version) -> bool {
    let Some(lower) = parse_loose(raw) else {
        return false;
    };
    if version < &lower {
        return false;
    }
    let upper = if component_count(raw) >= 3 {
        Version::new(lower.major, lower.minor + 1, 0)
    } else {
        Version::new(lower.major + 1, 0, 0)
    };
    version < &upper
}

/// `1.*` allows any `1.x`, `1.2.*` allows `1.2.x`.
/// `1.*` cho phép mọi `1.x`, `1.2.*` cho phép `1.2.x`.
fn wildcard(term: &str, version: &Version) -> bool {
    let cleaned = term.replace(".x", ".*").replace(".X", ".*");
    let head = cleaned.trim_end_matches('*').trim_end_matches('.');
    let parts: Vec<&str> = head.split('.').filter(|p| !p.is_empty()).collect();
    match parts.as_slice() {
        [] => true,
        [major] => major
            .parse::<u64>()
            .is_ok_and(|major| version.major == major),
        [major, minor, ..] => {
            let major_ok = major
                .parse::<u64>()
                .is_ok_and(|major| version.major == major);
            let minor_ok = minor
                .parse::<u64>()
                .is_ok_and(|minor| version.minor == minor);
            major_ok && minor_ok
        }
    }
}

/// Operators compare with Composer's zero-padding: `>=1.2` means `>=1.2.0`.
/// Toán tử so sánh theo kiểu bù số 0 của Composer: `>=1.2` là `>=1.2.0`.
/// Pre-release lower bounds (`>=1.0-RC1`) keep their `pre` segment.
/// Cận dưới pre-release giữ segment `pre`.
fn parse_loose(raw: &str) -> Option<Version> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    Version::parse(trimmed).ok()
}

fn component_count(raw: &str) -> usize {
    let base = raw
        .trim()
        .split(['-', '+'])
        .next()
        .unwrap_or("")
        .trim_start_matches(['v', 'V']);
    if base.is_empty() {
        return 0;
    }
    base.split('.').count()
}

/// A padded exact match must not carry a pre-release, build or post segment.
/// Khớp exact có bù số 0 không được mang pre/build/post.
fn no_tail(version: &Version) -> bool {
    version.pre.is_none() && version.post.is_none() && version.dev.is_none()
}

#[cfg(test)]
#[path = "test/version_constraints.rs"]
mod tests;
