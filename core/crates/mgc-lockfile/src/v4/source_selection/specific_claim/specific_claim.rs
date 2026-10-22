//! Choose the most-specific matching source and reject ties.
//! Chọn source khớp đặc hiệu nhất và từ chối tie.

use super::super::super::source_ref::SourceRef;
use super::super::error::SourceSelectionError;

pub(in crate::v4::source_selection) fn select_specific_claim<'a>(
    package: &str,
    matches: &[(&'a SourceRef, u8)],
    max_spec: u8,
) -> Result<&'a SourceRef, SourceSelectionError> {
    let mut top: Vec<&SourceRef> = matches
        .iter()
        .filter(|(_, r)| *r == max_spec)
        .map(|(s, _)| *s)
        .collect();
    top.sort_by_key(|s| s.id.clone());
    top.dedup_by_key(|s| s.id.clone());
    if top.len() > 1 {
        return Err(SourceSelectionError::AmbiguousSource {
            package: package.to_string(),
        });
    }
    // Specific branch needs no override check: the winner IS the maximum
    // specificity, so no trusted source can claim this package above it
    // (equal rank would have tied into AmbiguousSource above).
    // (Nhánh specific không cần check override: người thắng ĐÃ là độ đặc
    // hiệu tối đa.)
    Ok(top[0])
}

#[cfg(test)]
#[path = "test/specific_claim.rs"]
mod tests;
