//! `peer_validation.rs` — Resolved peer validation / kiểm tra peer đã resolve.
use super::super::types::{ResolutionIndex, ResolutionSnapshot};
use mgc_types::{MgError, MgResult, VersionRange};

pub(in crate::adapter::resolution) fn validate_required_peers(
    snapshot: &ResolutionSnapshot,
    index: &ResolutionIndex<'_>,
) -> MgResult<()> {
    for resolution in &snapshot.resolutions {
        let Some(version_metadata) = snapshot
            .metadata
            .get(resolution.package_id.name_str())
            .and_then(|package| {
                package
                    .versions
                    .get(&resolution.package_id.version().to_string())
            })
        else {
            continue;
        };
        let Some(peers) = version_metadata.peer_dependencies.as_ref() else {
            continue;
        };
        for (peer_name, peer_range) in peers {
            if version_metadata
                .peer_dependencies_meta
                .get(peer_name.as_str())
                .is_some_and(|metadata| metadata.optional)
            {
                continue;
            }
            let constraint = VersionRange::parse(peer_range).map_err(|_| {
                MgError::DependencyConflict(format!(
                    "invalid peer dependency range '{}' for '{}' required by '{}'",
                    peer_range, peer_name, resolution.package_id
                ))
            })?;
            if !index.get(peer_name.as_str()).is_some_and(|candidates| {
                candidates
                    .iter()
                    .any(|candidate| constraint.matches(candidate.package_id.version()))
            }) {
                return Err(MgError::DependencyConflict(format!(
                    "no resolved version of peer dependency '{}' satisfies '{}' required by '{}'",
                    peer_name, peer_range, resolution.package_id
                )));
            }
        }
    }
    Ok(())
}
