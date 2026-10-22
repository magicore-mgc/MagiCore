//! Plans Web store pruning before any deletion and renders truthful byte semantics.
//! Lập kế hoạch prune Web store trước khi xóa và hiển thị đúng ý nghĩa số byte.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::super::command_types::PruneReport;
use super::super::store_paths::store_cache_root_for;
use super::candidate_plan::plan_candidates;
use super::error::PruneError;
use super::root_safety::{
    existing_database_file, existing_store_cache_root, existing_vstore_root,
    validate_existing_lock_directory,
};

pub(in crate::commands::store) fn prune_unreferenced(
    project_root: &Path,
    dry_run: bool,
) -> Result<PruneReport> {
    let Some(initial_cache_root) = existing_store_cache_root(project_root)? else {
        return Ok(empty_report(dry_run));
    };
    let Some(initial_database) = existing_database_file(&initial_cache_root)? else {
        // `Database::open` creates and initializes missing files, so a missing DB is a true no-op.
        // `Database::open` tạo và khởi tạo file thiếu, vì vậy DB không tồn tại phải là no-op thật.
        return Ok(empty_report(dry_run));
    };
    let initial_vstore_root = existing_vstore_root(project_root)?;
    validate_existing_lock_directory(&initial_cache_root)?;

    // Match Web install's exact Layout::root() lock key and locks directory.
    // Khớp chính xác lock key Layout::root() và thư mục locks của Web install.
    let _project_lock = super::super::acquire_maintenance_lock(project_root)?;

    let Some(cache_root) = existing_store_cache_root(project_root)? else {
        return Err(PruneError::StoreRootChanged {
            path: display_path(&store_cache_root_for(project_root)),
        }
        .into());
    };
    if cache_root != initial_cache_root {
        return Err(PruneError::StoreRootChanged {
            path: display_path(&cache_root),
        }
        .into());
    }
    validate_existing_lock_directory(&cache_root)?;
    let Some(database_path) = existing_database_file(&cache_root)? else {
        return Err(PruneError::StoreRootChanged {
            path: display_path(&initial_database),
        }
        .into());
    };
    if database_path != initial_database {
        return Err(PruneError::StoreRootChanged {
            path: display_path(&database_path),
        }
        .into());
    }
    let vstore_root = existing_vstore_root(project_root)?;
    if vstore_root != initial_vstore_root {
        let changed_root = vstore_root
            .as_deref()
            .or(initial_vstore_root.as_deref())
            .unwrap_or(project_root);
        return Err(PruneError::StoreRootChanged {
            path: display_path(changed_root),
        }
        .into());
    }

    let database = if dry_run {
        mgc_store::Database::open_read_only(&database_path)
            .context("open project Web store without migration for dry-run")?
    } else {
        mgc_store::Database::open_existing(&database_path)
            .context("open project Web store for pruning")?
    };
    let unreferenced = database
        .list_unreferenced()
        .context("list unreferenced Web store packages")?;
    let (plan, projected_bytes) = plan_candidates(unreferenced, vstore_root.as_deref())?;

    let mut report = PruneReport {
        unreferenced: plan
            .iter()
            .map(|candidate| candidate.id.to_string())
            .collect(),
        removed: Vec::with_capacity(plan.len()),
        removed_bytes: 0,
        dry_run,
    };

    if dry_run {
        report.removed = plan
            .iter()
            .map(|candidate| candidate.id.to_string())
            .collect();
        report.removed_bytes = projected_bytes;
        return Ok(report);
    }

    for candidate in plan {
        if let Some(directory) = candidate.directory {
            let total_after_delete = report
                .removed_bytes
                .checked_add(candidate.logical_bytes)
                .ok_or_else(|| PruneError::CompletedPayloadOverflow {
                    package: candidate.id.to_string(),
                })?;
            fs::remove_dir_all(&directory).map_err(|source| {
                PruneError::RemovePackageDirectory {
                    path: display_path(&directory),
                    source,
                }
            })?;
            database
                .remove_package(&candidate.id)
                .with_context(|| format!("remove package record {}", candidate.id))?;
            report.removed_bytes = total_after_delete;
        } else {
            database
                .remove_package(&candidate.id)
                .with_context(|| format!("remove package record {}", candidate.id))?;
        }
        report.removed.push(candidate.id.to_string());
    }
    Ok(report)
}

pub(in crate::commands::store) fn run_prune(
    project_root: &Path,
    dry_run: bool,
    json: bool,
) -> Result<()> {
    let report = prune_unreferenced(project_root, dry_run)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!(
            "Project Web store prune {}: {} unreferenced package(s)",
            if dry_run { "(dry-run)" } else { "(pruned)" },
            report.unreferenced.len()
        );
        for id in &report.removed {
            println!(
                "  {} {id}",
                if dry_run { "would remove" } else { "removed" }
            );
        }
        if dry_run {
            println!(
                "  projected logical payload: {} bytes",
                report.removed_bytes
            );
        } else {
            println!(
                "  logical payload in removed directories: {} bytes",
                report.removed_bytes
            );
        }
    }
    Ok(())
}

fn empty_report(dry_run: bool) -> PruneReport {
    PruneReport {
        unreferenced: Vec::new(),
        removed: Vec::new(),
        removed_bytes: 0,
        dry_run,
    }
}

fn display_path(path: &Path) -> String {
    path.display().to_string()
}
