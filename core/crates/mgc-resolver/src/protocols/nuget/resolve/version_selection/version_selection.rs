//! Fetch and choose the lowest listed NuGet version matching a requirement.
//! Tải và chọn version NuGet listed thấp nhất khớp yêu cầu.

use crate::protocols::nuget::{NuGetProtocol, nuget_matches};
use mgc_types::{MgError, MgResult, Version};
use serde_json::Value;

/// Fetch the flat-container version list and fail closed when no listed candidate matches.
/// Tải danh sách flat-container và fail-closed nếu không có candidate listed khớp.
pub(in crate::protocols::nuget::resolve) async fn select_listed_version(
    protocol: &NuGetProtocol,
    id: &str,
    id_lower: &str,
    range: &str,
) -> MgResult<String> {
    // Flat container version list — danh sách version flat container.
    let versions_url = format!("{}/{id_lower}/index.json", protocol.package_base);
    let (status, body) = protocol.get_text(&versions_url).await?;
    if !(200..300).contains(&status) {
        return Err(MgError::Network(format!(
            "GET {versions_url} returned {status} — no versions for {id} (fail-closed)"
        )));
    }
    let doc: Value = serde_json::from_str(&body)
        .map_err(|e| MgError::Other(format!("parse flat container index failed: {e}")))?;
    let versions: Vec<&str> = doc
        .get("versions")
        .and_then(Value::as_array)
        .map(|values| values.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();

    match lowest_matching_version(&versions, range) {
        Some(version) => Ok(version),
        None => {
            if !versions.is_empty() {
                eprintln!(
                    "WARNING: {id}@{range} only matches versions with no listed registration \
                     — skipping (fail-closed)"
                );
            }
            Err(MgError::Other(format!(
                "no listed version of {id} matches range '{range}'"
            )))
        }
    }
}

/// Return the first raw spelling of the lowest parseable version matching the range.
/// Trả spelling raw đầu tiên của version parse được thấp nhất khớp range.
fn lowest_matching_version(versions: &[&str], range: &str) -> Option<String> {
    let mut best: Option<(Version, String)> = None;
    for raw in versions {
        let Ok(version) = Version::parse(raw) else {
            continue;
        };
        if !nuget_matches(range, &version) {
            continue;
        }
        if best.as_ref().is_none_or(|(current, _)| version < *current) {
            best = Some((version, (*raw).to_string()));
        }
    }
    best.map(|(_, raw)| raw)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
