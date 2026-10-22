//! Select registry sources under v4 claim policy.
//! Chọn registry source theo policy claim v4.

use super::super::source_claim::{claim_matches, claim_specificity};
use super::super::source_ref::SourceRef;
use super::error::SourceSelectionError;

/// Fallback priority for the labeled "unknown" source: higher than any
/// real priority, low enough for TOML's i64-only integers.
/// (Priority cho nguồn "unknown": cao hơn mọi priority thật, đủ thấp cho
/// integer i64-only của TOML.)
pub const UNKNOWN_SOURCE_PRIORITY: u64 = 1_000_000_000;

/// Specificity-first source selection (§5.3): exact/specific claims beat
/// catch-all fallbacks; equal-specificity conflicts across sources fail;
/// public sources never override trusted specific claims.
/// Chọn nguồn theo độ đặc hiệu trước (§5.3).
pub fn select_source<'a>(
    package: &str,
    sources: &'a [SourceRef],
) -> Result<&'a SourceRef, SourceSelectionError> {
    let mut matches: Vec<(&'a SourceRef, u8)> = Vec::new();
    for source in sources {
        let mut best: Option<u8> = None;
        for pattern in &source.claims {
            if claim_matches(pattern, package) {
                let rank = claim_specificity(pattern);
                best = Some(best.map_or(rank, |b: u8| b.max(rank)));
            }
        }
        if let Some(rank) = best {
            matches.push((source, rank));
        }
    }

    if matches.is_empty() {
        return super::unmatched_fallback::select_unmatched_fallback(package, sources);
    }

    let max_spec = matches.iter().map(|(_, r)| *r).max().unwrap_or(0);

    if max_spec == 0 {
        return super::catch_all_fallback::select_catch_all_fallback(package, sources, &matches);
    }
    super::specific_claim::select_specific_claim(package, &matches, max_spec)
}

#[cfg(test)]
#[path = "test/source_selection.rs"]
mod tests;
