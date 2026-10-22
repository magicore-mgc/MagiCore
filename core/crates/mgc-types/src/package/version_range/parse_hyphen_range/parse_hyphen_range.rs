//! Implements the `parse_hyphen_range` version-range operation.
//! Cài đặt thao tác version range `parse_hyphen_range`.

use crate::version::Version;

pub(in crate::package::version_range) fn parse_hyphen_range(
    range: &str,
) -> Option<(Version, Version)> {
    let trimmed = range.trim();
    let parts: Vec<&str> = trimmed.splitn(3, |c: char| c.is_whitespace()).collect();
    if parts.len() != 3 || parts[1] != "-" {
        return None;
    }
    let low = Version::parse(parts[0]).ok()?;
    let high = Version::parse(parts[2]).ok()?;
    Some((low, high))
}
