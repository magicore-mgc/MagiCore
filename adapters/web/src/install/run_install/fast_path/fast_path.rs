//! No-op install detection and lock reconciliation.
//! Dò install không đổi và đồng bộ lock.

use crate::install::run_install::state::InstallState;
use crate::lockfile::write_web_lockfile_with_state;
use mgc_types::MgResult;
use rayon::prelude::*;

pub(in crate::install::run_install) fn try_noop(state: &mut InstallState<'_>) -> MgResult<bool> {
    if state.profile.enabled && !state.all_root_matched {
        let missing: Vec<_> = state
            .root_packages
            .par_iter()
            .filter(|package| {
                !crate::lockfile::installed_package_matches(
                    &state.node_modules.join(package.id.name().as_str()),
                    &package.id,
                )
            })
            .map(|package| package.id.to_string())
            .collect();
        eprintln!("[magicore:web:noop-profile] root_matched=false missing={missing:?}");
    }

    if state.all_root_matched
        && state.opts.force_install.is_empty()
        && !state.graph.packages.is_empty()
    {
        let all_vstore_matched = state.graph.packages.par_iter().all(|package| {
            crate::lockfile::installed_package_matches(
                &crate::install::materialize::strict_vstore_package_dir(
                    &state.node_modules,
                    &package.id,
                ),
                &package.id,
            )
        });
        if state.profile.enabled && !all_vstore_matched {
            let missing: Vec<_> = state
                .graph
                .packages
                .par_iter()
                .filter(|package| {
                    !crate::lockfile::installed_package_matches(
                        &crate::install::materialize::strict_vstore_package_dir(
                            &state.node_modules,
                            &package.id,
                        ),
                        &package.id,
                    )
                })
                .map(|package| package.id.to_string())
                .collect();
            eprintln!("[magicore:web:noop-profile] vstore_matched=false missing={missing:?}");
        }
        if all_vstore_matched {
            // Warm no-op still reconciles lock state with the accepted graph.
            // Warm no-op vẫn đồng bộ lock với graph được chấp nhận.
            write_web_lockfile_with_state(state.project_root, state.graph, state.registry_url)?;
            state.summary.duration_ms = state.start.elapsed().as_millis() as u64;
            return Ok(true);
        }
    }
    Ok(false)
}
