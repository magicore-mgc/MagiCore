//! Composer package-name validation (`vendor/package`).
//! Kiểm tra tên package Composer (`vendor/package`).

use super::super::error::ComposerError;
use mgc_types::MgResult;

const MAX_NAME_LEN: usize = 214;

/// Validate and lowercase a Composer package name.
/// Composer names are `vendor/package`; both segments are lowercase
/// `[a-z0-9_.-]` and must start with a letter or digit.
/// Kiểm tra và hạ chữ thường tên package Composer. Hai segment là chữ
/// thường `[a-z0-9_.-]` và bắt đầu bằng chữ cái/số.
pub(in crate::protocols::composer) fn validate_composer_package_name(
    name: &str,
) -> MgResult<String> {
    let invalid = || ComposerError::InvalidPackageName(name.to_string()).into_mg_error();
    let lower = name.trim().to_ascii_lowercase();
    if lower.is_empty() || lower.len() > MAX_NAME_LEN {
        return Err(invalid());
    }
    let Some((vendor, package)) = lower.split_once('/') else {
        return Err(invalid());
    };
    if package.contains('/') || !segment_is_valid(vendor) || !segment_is_valid(package) {
        return Err(invalid());
    }
    Ok(lower)
}

fn segment_is_valid(segment: &str) -> bool {
    if segment.is_empty() || segment.len() > MAX_NAME_LEN {
        return false;
    }
    let mut chars = segment.chars();
    let starts_well = chars
        .next()
        .is_some_and(|first| first.is_ascii_alphanumeric());
    starts_well && chars.all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '-'))
}

#[cfg(test)]
#[path = "test/package_name.rs"]
mod tests;
