//! Select matching wildcard fallback claims with trusted-source guard.
//! Chọn claim wildcard fallback khớp và giữ chặn source tin cậy.

use super::super::super::source_claim::claim_matches;
use super::super::super::source_ref::SourceRef;
use super::super::error::SourceSelectionError;

pub(in crate::v4::source_selection) fn select_catch_all_fallback<'a>(
    package: &str,
    sources: &'a [SourceRef],
    matches: &[(&'a SourceRef, u8)],
) -> Result<&'a SourceRef, SourceSelectionError> {
    let top: Vec<&&SourceRef> = matches.iter().map(|(s, _)| s).collect();
    let min_priority = top.iter().map(|s| s.priority).min().unwrap_or(u64::MAX);
    let winners: Vec<&&SourceRef> = top
        .into_iter()
        .filter(|s| s.priority == min_priority)
        .collect();
    if winners.len() > 1 {
        return Err(SourceSelectionError::AmbiguousFallback {
            package: package.to_string(),
        });
    }
    let chosen = winners[0];
    // Fallback-level anti-confusion: a winning UNTRUSTED catch-all
    // loses to ANY trusted match — specificity-first already
    // guarantees no trusted source claims this package above rank 0,
    // so falling back to public while a trusted source matches is a
    // misconfiguration, failed loudly instead of resolved quietly.
    // (Chống nhầm lẫn ở tầng fallback: catch-all không-tin-cậy thắng
    // vẫn thua BẤT KỲ match tin-cậy nào — specificity-first đã đảm
    // bảo không có nguồn tin cậy nào claim package này trên hạng 0.)
    if !chosen.trusted
        && sources.iter().any(|s| {
            s.trusted && s.id != chosen.id && s.claims.iter().any(|p| claim_matches(p, package))
        })
    {
        return Err(SourceSelectionError::PublicOverridePrivate {
            package: package.to_string(),
            source: chosen.id.clone(),
        });
    }
    Ok(chosen)
}

#[cfg(test)]
#[path = "test/catch_all_fallback.rs"]
mod tests;
