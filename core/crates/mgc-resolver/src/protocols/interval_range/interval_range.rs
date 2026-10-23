//! Bracket interval-range matching shared by NuGet and Maven version constraints.
//! Khớp range dạng ngoặc dùng chung cho constraint version NuGet và Maven.

use mgc_types::Version;

/// Match a bracketed interval (`[1.0,2.0)`, `(1.0,)`, `[1.0]`) against a version.
/// Khớp interval dạng ngoặc với một version.
pub(crate) fn interval_matches(range: &str, version: &Version) -> bool {
    let Some(close) = range.rfind([')', ']']) else {
        return false;
    };
    let (open_ch, close_ch) = (range.as_bytes()[0] as char, range.as_bytes()[close] as char);
    let inner = &range[1..close];
    let bounds: Vec<&str> = inner.split(',').map(str::trim).collect();
    match bounds.len() {
        1 => {
            open_ch == '['
                && close_ch == ']'
                && Version::parse(bounds[0])
                    .map(|t| version == &t)
                    .unwrap_or(false)
        }
        2 => {
            let lower_ok = match (open_ch, bounds[0].is_empty()) {
                (_, true) => true,
                ('[', false) => Version::parse(bounds[0])
                    .map(|t| version >= &t)
                    .unwrap_or(false),
                ('(', false) => Version::parse(bounds[0])
                    .map(|t| version > &t)
                    .unwrap_or(false),
                _ => false,
            };
            let upper_ok = match (close_ch, bounds[1].is_empty()) {
                (_, true) => true,
                (']', false) => Version::parse(bounds[1])
                    .map(|t| version <= &t)
                    .unwrap_or(false),
                (')', false) => Version::parse(bounds[1])
                    .map(|t| version < &t)
                    .unwrap_or(false),
                _ => false,
            };
            lower_ok && upper_ok
        }
        _ => false,
    }
}
