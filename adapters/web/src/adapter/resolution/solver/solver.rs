//! `solver.rs` — Dependency solving and metadata acquisition / giải dependency và lấy metadata.
use super::super::super::WebAdapter;
use super::super::types::ResolutionSnapshot;
use crate::profile::ResolveProfile;
use mgc_types::{MgError, MgResult, PackageName};

pub(in crate::adapter::resolution) async fn solve(
    adapter: &WebAdapter,
    wanted: &[(PackageName, String)],
    profile: &mut ResolveProfile,
) -> MgResult<ResolutionSnapshot> {
    let solve_started_at = std::time::Instant::now();
    let result = adapter
        .resolver
        .solve_with_roots(wanted)
        .await
        .map_err(|e| MgError::DependencyConflict(e.message))?;
    profile.mark("solver_solve", solve_started_at);

    let metadata_started_at = std::time::Instant::now();
    let metadata = adapter
        .provider
        .prefetch_resolution_metadata(
            &result
                .result
                .resolutions
                .iter()
                .map(|r| r.package_id.name().clone())
                .collect::<Vec<_>>(),
        )
        .await
        .map_err(|err| MgError::Network(err.to_string()))?;
    profile.mark("prefetch_resolution_metadata", metadata_started_at);
    Ok(ResolutionSnapshot {
        resolutions: result.result.resolutions,
        root_resolutions: result.root_resolutions,
        metadata,
    })
}
