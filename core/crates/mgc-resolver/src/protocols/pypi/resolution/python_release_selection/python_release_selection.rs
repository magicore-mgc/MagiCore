//! Select a release compatible with the declared consumer Python.
//! Chọn release tương thích consumer Python được khai báo.

use super::super::types::{ReleaseCandidate, SelectedRelease};
use crate::protocols::pypi::{
    PypiInfo, PypiJson, PypiProtocol, consumer_python, requires_python_allows, select_file,
};
use mgc_types::{MgError, MgResult};

pub(in crate::protocols::pypi::resolution) async fn select_compatible_release(
    protocol: &PypiProtocol,
    name: &str,
    range: &str,
    candidates: Vec<ReleaseCandidate>,
) -> MgResult<SelectedRelease> {
    let consumer = consumer_python();
    let selected = match consumer {
        None => {
            let candidate = candidates
                .into_iter()
                .next()
                .ok_or_else(|| MgError::Other(format!("no version of {name} matches '{range}'")))?;
            SelectedRelease {
                version: candidate.version,
                files: candidate.files,
                consumer: None,
                version_info: None,
            }
        }
        Some((major, minor)) => {
            let mut selected: Option<(ReleaseCandidate, PypiInfo)> = None;
            let mut last_excluded: Option<String> = None;
            for candidate in candidates {
                let version_url = format!(
                    "{}/pypi/{name}/{}/json",
                    protocol.index_url, candidate.version
                );
                let version_body = protocol.get_text(&version_url).await.map_err(|error| {
                    MgError::Network(format!(
                        "cannot determine Python compatibility for {name} {}: {error}",
                        candidate.version
                    ))
                })?;
                let version_doc: PypiJson =
                    serde_json::from_str(&version_body).map_err(|error| {
                        MgError::Other(format!(
                            "parse Python compatibility metadata for {name} {}: {error}",
                            candidate.version
                        ))
                    })?;
                let required = version_doc.info.requires_python.clone().unwrap_or_default();
                match candidate_incompatibility(&candidate, &required, major, minor) {
                    Some(reason) => last_excluded = Some(reason),
                    None => {
                        selected = Some((candidate, version_doc.info));
                        break;
                    }
                }
            }
            let (candidate, version_info) = selected.ok_or_else(|| {
                MgError::Other(format!(
                    "no version of {name} matches '{range}' on Python {major}.{minor}{}",
                    last_excluded
                        .map(|exclusion| format!(" (newest excluded: {exclusion})"))
                        .unwrap_or_default()
                ))
            })?;
            SelectedRelease {
                version: candidate.version,
                files: candidate.files,
                consumer: Some((major, minor)),
                version_info: Some(version_info),
            }
        }
    };
    Ok(selected)
}

fn candidate_incompatibility(
    candidate: &ReleaseCandidate,
    required: &str,
    major: u64,
    minor: u64,
) -> Option<String> {
    if !required.trim().is_empty() && !requires_python_allows(required, major, minor) {
        return Some(format!("{} requires {required}", candidate.version));
    }
    if select_file(&candidate.files, Some((major, minor))).is_none() {
        return Some(format!(
            "{} has no artifact compatible with Python {major}.{minor}",
            candidate.version
        ));
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
