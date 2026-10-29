//! Final reference-set commit and CAS generation promotion.
//! Commit refset cuối và promote CAS generation.

use crate::install::run_install::state::InstallState;
use mgc_types::{MgError, MgResult};

pub(in crate::install::run_install) fn finish(state: &mut InstallState<'_>) -> MgResult<()> {
    let project_root_str = state.project_root.to_string_lossy().to_string();
    if let Some(database) = state.database.as_ref() {
        mgc_store::failpoint::hit("before-commit");
        // P-2.4: the reference set is replaced atomically, so a concurrent
        // prune never observes this project referencing nothing.
        // (P-2.4: tập tham chiếu được thay nguyên tử, nên prune đồng thời không
        // bao giờ thấy project này không tham chiếu gì.)
        let ids: Vec<&mgc_types::PackageId> = state
            .graph
            .packages
            .iter()
            .map(|package| &package.id)
            .collect();
        database
            .replace_refs(&project_root_str, ids)
            .map_err(|error| MgError::Store(error.to_string()))?;

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
