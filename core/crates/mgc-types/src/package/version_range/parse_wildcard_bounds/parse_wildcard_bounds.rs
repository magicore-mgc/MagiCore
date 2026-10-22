//! Implements the `parse_wildcard_bounds` version-range operation.
//! Cài đặt thao tác version range `parse_wildcard_bounds`.

use crate::version::Version;

pub(in crate::package::version_range) fn parse_wildcard_bounds(
    range: &str,
) -> Option<(Version, Version)> {
    let trimmed = range
        .trim_start_matches('=')
        .trim_start_matches('^')
        .trim_start_matches('~')
        .trim_start_matches(">=")
        .trim_start_matches("<=")
        .trim_start_matches('>')
        .trim_start_matches('<')
        .trim();

    if !trimmed.contains('x') && !trimmed.contains('X') {
        return None;
    }
    let parts: Vec<&str> = trimmed.split('.').collect();
    match parts.len() {
        1 if parts[0] == "x" || parts[0] == "X" => {
            Some((Version::new(0, 0, 0), Version::new(u64::MAX, 0, 0)))
        }
        2 => {
            let major: u64 = parts[0].parse().ok()?;
            if parts[1] == "x" || parts[1] == "X" {
                Some((Version::new(major, 0, 0), Version::new(major + 1, 0, 0)))
            } else {
                None
            }
        }
        3 => {
            let major: u64 = parts[0].parse().ok()?;
            let minor: u64 = parts[1].parse().ok()?;
            if parts[2] == "x" || parts[2] == "X" {
                Some((
                    Version::new(major, minor, 0),
                    Version::new(major, minor + 1, 0),
                ))
            } else {
                None
            }
        }
        _ => None,
    }
}
