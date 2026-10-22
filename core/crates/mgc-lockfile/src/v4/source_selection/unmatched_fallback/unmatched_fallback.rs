//! Choose the minimum-priority source when no claim matches.
//! Chọn source priority nhỏ nhất khi không claim nào khớp.

use super::super::super::source_ref::SourceRef;
use super::super::error::SourceSelectionError;

pub(in crate::v4::source_selection) fn select_unmatched_fallback<'a>(
    package: &str,
    sources: &'a [SourceRef],
) -> Result<&'a SourceRef, SourceSelectionError> {
    // Sole fallback road: lowest priority number wins.
    // (Đường fallback duy nhất: số priority nhỏ nhất thắng.)
    let min_priority = sources
        .iter()
        .map(|source| source.priority)
        .min()
        .ok_or_else(|| SourceSelectionError::AmbiguousFallback {
            package: package.to_string(),
        })?;
    let mut winners = sources
        .iter()
        .filter(|source| source.priority == min_priority);
    let winner = winners
        .next()
        .ok_or_else(|| SourceSelectionError::AmbiguousFallback {
            package: package.to_string(),
        })?;
    if winners.next().is_some() {
        return Err(SourceSelectionError::AmbiguousFallback {
            package: package.to_string(),
        });
    }
    Ok(winner)
}

#[cfg(test)]
#[path = "test/unmatched_fallback.rs"]
mod tests;
