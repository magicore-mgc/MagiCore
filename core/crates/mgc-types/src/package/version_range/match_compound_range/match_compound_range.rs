//! Implements the `match_compound_range` version-range operation.
//! Cài đặt thao tác version range `match_compound_range`.

use super::super::{match_single_range, parse_hyphen_range};
use crate::version::Version;

pub(in crate::package::version_range) fn match_compound_range(
    range: &str,
    version: &Version,
) -> bool {
    let trimmed = range.trim();
    if trimmed.is_empty() {
        return false;
    }
    if trimmed == "*" {
        return true;
    }

    // Hyphen range: "1 - 2" → >=1.0.0 <=2.0.0, "1.2.3 - 2.3.4" → >=1.2.3 <=2.3.4
    if let Some((low, high)) = parse_hyphen_range(trimmed) {
        return version.precedence_cmp(&low) != std::cmp::Ordering::Less
            && version.precedence_cmp(&high) != std::cmp::Ordering::Greater;
    }

    let mut parts = Vec::new();
    let mut tokens = trimmed.split_whitespace().peekable();
    while let Some(token) = tokens.next() {
        if matches!(token, ">=" | "<=" | ">" | "<" | "=" | "==" | "^" | "~") {
            if let Some(next) = tokens.next() {
                parts.push(format!("{token}{next}"));
            } else {
                return false;
            }
        } else {
            parts.push(token.to_string());
        }
    }

    parts
        .iter()
        .all(|part| match_single_range(part.as_str(), version))
}
