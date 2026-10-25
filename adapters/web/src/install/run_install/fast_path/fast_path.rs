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
        // Verify content only where a marker exists. MagiCore always writes one
        // into the virtual store, so a store it produced is checked byte for
        // byte; a tree laid down by another installer has no marker and keeps
        // the version-only contract rather than being reinstalled on sight.
        // Chỉ verify nội dung khi có marker. MagiCore luôn ghi marker vào virtual
        // store, nên store do nó tạo được kiểm từng byte; cây do installer khác
        // dựng không có marker và giữ hợp đồng chỉ-version thay vì bị cài lại.
        // A tree missing its virtual store cannot be intact, whatever the
        // per-package checks conclude: `node_modules` containing only `.bin` is
        // the shape a killed rollback leaves behind.
        // Cây thiếu virtual store không thể nguyên vẹn, bất kể kiểm từng package
        // kết luận gì: `node_modules` chỉ có `.bin` là dạng rollback bị kill để
        // lại.
        let vstore_present = state.node_modules.join(".magicore").is_dir();
        let all_vstore_matched = vstore_present
            && state.graph.packages.par_iter().all(|package| {
                let root = crate::install::materialize::strict_vstore_package_dir(
                    &state.node_modules,
                    &package.id,
                );
                if !crate::lockfile::installed_package_matches(&root, &package.id) {
                    return false;
                }
                match crate::install::package_marker::read_extracted_package_marker(&root) {
                    Ok(Some(marker)) => {
                        crate::install::package_marker::extracted_content_matches(&root, &marker)
                            .unwrap_or(false)
                    }
                    _ => true,
                }
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
