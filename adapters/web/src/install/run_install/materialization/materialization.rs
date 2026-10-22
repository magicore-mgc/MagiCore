//! Ordered coordinator for Web package materialization.
//! Điều phối theo thứ tự materialize package Web.

use crate::install::TreeBackup;
use crate::install::run_install::state::InstallState;
use mgc_types::MgResult;

pub(in crate::install::run_install) async fn materialize(
    state: &mut InstallState<'_>,
) -> MgResult<()> {
    let tree_will_change = state
        .root_packages
        .iter()
        .any(|package| !state.already_materialized.contains(&package.id))
        || !state.opts.force_install.is_empty();
    state.tree_backup = Some(if tree_will_change {
        TreeBackup::take(&state.node_modules)?
    } else {
        TreeBackup::none(&state.node_modules)
    });

    if state.opts.legacy_flat {
        super::legacy_flat::materialize_root_packages(state)?;
    } else {
        let root_packages = state.root_packages.clone();
        let database = state.database.as_ref();
        let summary = &mut state.summary;
        let opts = &state.opts;
        let already_materialized = &state.already_materialized;
        for package in &root_packages {
            if let Some(database) = database {
                database
                    .insert_package(
                        &package.id,
                        if package.integrity.is_empty() {
                            None
                        } else {
                            Some(package.integrity.as_str())
                        },
                    )
                    .map_err(|error| mgc_types::MgError::Store(error.to_string()))?;
            }
            if !opts.incremental || !already_materialized.contains(&package.id) {
                summary.added.push(package.id.clone());
            }
        }
    }
    state.profile.mark("materialize_root_packages", state.start);

    let affected_root_bin_links = super::dependency_graph::materialize_dependencies(state).await?;
    state
        .profile
        .mark("materialize_dependency_graph", state.start);
    super::finalize::finish(state, affected_root_bin_links)?;
    Ok(())
}
