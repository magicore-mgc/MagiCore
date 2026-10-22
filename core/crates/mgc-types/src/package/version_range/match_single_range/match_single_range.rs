//! Implements the `match_single_range` version-range operation.
//! Cài đặt thao tác version range `match_single_range`.

use super::super::parse_wildcard_bounds;
use crate::version::Version;

pub(in crate::package::version_range) fn match_single_range(
    range: &str,
    version: &Version,
) -> bool {
    if range == "*" {
        return true;
    }
    if let Some(target) = range
        .trim_start_matches('=')
        .split_whitespace()
        .next()
        .and_then(|s| Version::parse(s).ok())
    {
        return version.precedence_cmp(&target) == std::cmp::Ordering::Equal;
    }
    if let Some(target) = range.strip_prefix('^').and_then(|s| Version::parse(s).ok()) {
        // Caret follows semver 0.x rules: ^1.2.3 := >=1.2.3 <2.0.0,
        // ^0.2.3 := >=0.2.3 <0.3.0, ^0.0.3 := =0.0.3. The old
        // major-only check accepted e.g. 0.28.2 for ^0.25.0.
        // (Caret đúng semver 0.x.)
        if target.major > 0 {
            return version.major == target.major
                && version.precedence_cmp(&target) != std::cmp::Ordering::Less;
        }
        if target.minor > 0 {
            return version.major == 0
                && version.minor == target.minor
                && version.precedence_cmp(&target) != std::cmp::Ordering::Less;
        }
        return version.major == 0
            && version.minor == 0
            && version.patch == target.patch
            && version.precedence_cmp(&target) != std::cmp::Ordering::Less;
    }
    if let Some(target) = range.strip_prefix('~').and_then(|s| Version::parse(s).ok()) {
        return version.major == target.major
            && version.minor == target.minor
            && version.precedence_cmp(&target) != std::cmp::Ordering::Less;
    }
    // Enhanced: Handle wildcard ranges with comparison operators (>=22.x, <=24.x)
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if (range.starts_with(">=")
        || range.starts_with("<=")
        || range.starts_with('>')
        || range.starts_with('<'))
        && let Some((low, high)) = parse_wildcard_bounds(range)
    {
        // For >= and >, check lower bound; for <= and <, check upper bound
        if range.starts_with(">=") {
            return version.precedence_cmp(&low) != std::cmp::Ordering::Less;
        } else if range.starts_with(">") {
            return version.precedence_cmp(&low) == std::cmp::Ordering::Greater;
        } else if range.starts_with("<=") {
            return version.precedence_cmp(&high) == std::cmp::Ordering::Less;
        } else if range.starts_with("<") {
            return version.precedence_cmp(&low) == std::cmp::Ordering::Less;
        }
    }
    if let Some(target) = range
        .strip_prefix(">=")
        .and_then(|s| Version::parse(s).ok())
    {
        return version.precedence_cmp(&target) != std::cmp::Ordering::Less;
    }
    if let Some(target) = range
        .strip_prefix("<=")
        .and_then(|s| Version::parse(s).ok())
    {
        return version.precedence_cmp(&target) != std::cmp::Ordering::Greater;
    }
    if let Some(target) = range.strip_prefix('>').and_then(|s| Version::parse(s).ok()) {
        return version.precedence_cmp(&target) == std::cmp::Ordering::Greater;
    }
    if let Some(target) = range.strip_prefix('<').and_then(|s| Version::parse(s).ok()) {
        return version.precedence_cmp(&target) == std::cmp::Ordering::Less;
    }
    // x-range wildcard: 12.x, 12.3.x, x
    if let Some((low, high)) = parse_wildcard_bounds(range) {
        return version.precedence_cmp(&low) != std::cmp::Ordering::Less
            && version.precedence_cmp(&high) == std::cmp::Ordering::Less;
    }
    Version::parse(range)
        .map(|target| version.precedence_cmp(&target) == std::cmp::Ordering::Equal)
        .unwrap_or(false)
}
