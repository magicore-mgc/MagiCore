//! Final reference-set commit and CAS generation promotion.
//! Commit refset cuối và promote CAS generation.

use crate::install::run_install::state::InstallState;
use mgc_types::{MgError, MgResult};

pub(in crate::install::run_install) fn finish(state: &mut InstallState<'_>) -> MgResult<()> {
    let project_root_str = state.project_root.to_string_lossy().to_string();
    if let Some(database) = state.database.as_ref() {
        mgc_store::failpoint::hit("before-commit");
        database
            .clear_all_refs(&project_root_str)
            .map_err(|error| MgError::Store(error.to_string()))?;
        for package in &state.graph.packages {
            database
                .set_ref(&project_root_str, &package.id)
                .map_err(|error| MgError::Store(error.to_string()))?;
        }

        // begin/claim/promote/abort all use the same per-store project key.
        // begin/claim/promote/abort dùng chung khóa project theo store.
        let cas_project_key = state.layout.root().to_string_lossy().to_string();
        mgc_store::failpoint::hit("before-promote");
        database
            .promote_cas_generation(&cas_project_key, state.cas_generation)
            .map_err(|error| MgError::Store(error.to_string()))?;
        mgc_store::failpoint::hit("after-generation-flip");
        if let Some(guard) = state.cas_generation_guard.take() {
            guard.disarm();
        }
        mgc_store::failpoint::hit("after-commit");
    }
    if state.opts.repair {
        crate::install::materialize::repair_dangling_symlinks(&state.node_modules)?;
    }

    state.summary.duration_ms = state.start.elapsed().as_millis() as u64;
    state.profile.flush(state.summary.duration_ms);
    Ok(())
}
