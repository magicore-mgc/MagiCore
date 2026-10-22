//! Extracts a safe minimum Dart language version.
//! Trích xuất phiên bản ngôn ngữ Dart tối thiểu an toàn.

/// Extract the Dart language version floor from constraints with an explicit
/// lower bound. Constraints without a safely understood floor leave this
/// optional field absent rather than inventing a language version.
pub(in crate::install::flutter_package_config) fn dart_language_version_floor(
    constraint: &str,
) -> Option<String> {
    for token in constraint.split_whitespace() {
        let candidate = token
            .strip_prefix(">=")
            .or_else(|| token.strip_prefix('^'))
            .or_else(|| token.strip_prefix('='));
        let Some(candidate) = candidate else { continue };
        let mut parts = candidate.trim().split('.');
        let major = parts.next()?.parse::<u64>().ok()?;
        let minor = parts.next()?.parse::<u64>().ok()?;
        if major > 0 || minor > 0 {
            return Some(format!("{major}.{minor}"));
        }
    }
    None
}
