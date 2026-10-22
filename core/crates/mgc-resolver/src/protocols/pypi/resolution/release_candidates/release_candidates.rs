//! Load, filter, order, and age-gate PyPI release candidates.
//! Tải, lọc, sắp xếp và áp dụng age gate cho release PyPI.

use super::super::types::ReleaseCandidate;
use crate::protocols::pypi::{AgeGateExclusion, PypiFile, PypiJson, PypiProtocol, pep440_matches};
use mgc_types::{MgError, MgResult, Version};
use std::collections::HashMap;
use time::OffsetDateTime;

pub(in crate::protocols::pypi::resolution) async fn load_candidates(
    protocol: &PypiProtocol,
    name: &str,
    range: &str,
) -> MgResult<Vec<ReleaseCandidate>> {
    let url = format!("{}/pypi/{name}/json", protocol.index_url);
    let body = protocol.get_text(&url).await?;
    let doc: PypiJson = serde_json::from_str(&body)
        .map_err(|error| MgError::Other(format!("parse pypi json failed: {error}")))?;
    let mut candidates = matching_candidates(&doc.releases, range);

    let mut excluded_too_young = 0usize;
    let mut excluded_unstamped = 0usize;
    if let Some(policy) = protocol.age_gate {
        let now = OffsetDateTime::now_utc();
        candidates.retain(|candidate| {
            match PypiProtocol::release_age_exclusion(
                &candidate.files,
                policy,
                now,
                name,
                &candidate.version,
            ) {
                Some(AgeGateExclusion::TooYoung) => {
                    excluded_too_young += 1;
                    false
                }
                Some(AgeGateExclusion::Unstamped) => {
                    excluded_unstamped += 1;
                    false
                }
                None => true,
            }
        });
    }
    if candidates.is_empty() && (excluded_too_young > 0 || excluded_unstamped > 0) {
        return Err(MgError::Other(format!(
            "minimum-release-age: all matching versions of {name} excluded ({} too young, {} unstamped); relax [security] min_release_age or explicitly allow missing timestamps",
            excluded_too_young, excluded_unstamped
        )));
    }

    Ok(candidates)
}

fn matching_candidates(
    releases: &HashMap<String, Vec<PypiFile>>,
    range: &str,
) -> Vec<ReleaseCandidate> {
    let mut candidates: Vec<ReleaseCandidate> = releases
        .iter()
        .filter(|(version, files)| {
            !files.is_empty()
                && Version::parse(version).is_ok_and(|parsed| pep440_matches(range, &parsed))
        })
        .filter_map(|(version, files)| {
            Version::parse(version).ok().map(|parsed| ReleaseCandidate {
                version: parsed,
                files: files.clone(),
            })
        })
        .collect();
    candidates.sort_by(|left, right| right.version.cmp(&left.version));
    candidates
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
