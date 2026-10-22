//! Policy evaluation, task draining, and rollback for package lifecycle scripts.
//! Đánh giá policy, chờ task và rollback khi script package lỗi.

use crate::install::TreeBackup;
use crate::install::drain_lifecycle_tasks;
use crate::install::run_install::state::InstallState;
use crate::install::script_policy::{
    decide_lifecycle_scripts, lifecycle_scripts_allowed, load_trust_policies,
};
use crate::lifecycle::LifecycleRunner;
use mgc_types::{MgError, MgResult};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

pub(in crate::install::run_install) async fn run(state: &mut InstallState<'_>) -> MgResult<()> {
    let mut tree_backup = state
        .tree_backup
        .take()
        .unwrap_or_else(|| TreeBackup::none(&state.node_modules));

    let file_policy = if !state.opts.ignore_scripts {
        Some(crate::install::script_policy::load_file_scripts_policy(state.project_root)
            .map_err(|error| MgError::Other(format!(
                "[scripts] policy parse failed — install aborted (use --ignore-scripts to skip): {error}"
            )))?)
    } else {
        None
    };

    if !state.opts.ignore_scripts {
        let trust_map = load_trust_policies(&state.layout)?;
        let blanket_scripts = state.opts.allow_scripts || lifecycle_scripts_allowed();
        let mut scripted_packages = Vec::new();
        for package in &state.packages_with_scripts {
            let scripts = crate::lifecycle::load_package_scripts(&package.directory)?;
            if !scripts.has_hooks() {
                continue;
            }
            let name = package.package_id.name_str();
            let version = package.package_id.version().to_string();
            match decide_lifecycle_scripts(
                &package.package_id,
                file_policy.as_ref().and_then(|policy| policy.as_ref()),
                &trust_map,
                blanket_scripts,
            ) {
                mgc_config::project::ScriptVerdict::Deny(reason) => {
                    eprintln!(
                        "[magicore] DENIED lifecycle scripts for {name}@{version} ({reason})"
                    );
                }
                mgc_config::project::ScriptVerdict::Allow(_) => {
                    scripted_packages.push((package.directory.clone(), scripts));
                }
                mgc_config::project::ScriptVerdict::Undecided => {
                    if blanket_scripts {
                        scripted_packages.push((package.directory.clone(), scripts));
                    } else {
                        eprintln!(
                            "[magicore] skipped lifecycle scripts for {name}@{version} — not approved. Approve with: mgc trust approve {name}"
                        );
                    }
                }
            }
        }

        let semaphore = Arc::new(Semaphore::new(8));
        let mut join_set = JoinSet::new();
        for (package_dir, scripts) in scripted_packages {
            let project_root = state.project_root.to_path_buf();
            let Ok(permit) = semaphore.clone().acquire_owned().await else {
                eprintln!("[magicore] warning: lifecycle semaphore closed");
                continue;
            };
            join_set.spawn(async move {
                let _permit = permit;
                LifecycleRunner::run_scripts_with_snapshot(&package_dir, &project_root, scripts)
            });
        }

        if let Some(script_err) = drain_lifecycle_tasks(&mut join_set).await {
            mgc_store::failpoint::hit("before-rollback-restore");
            let mut rollback_failures = Vec::new();
            if let Err(error) =
                crate::install::restore_prior_lock_result(state.project_root, &state.prior_lock)
            {
                rollback_failures.push(format!("lock restore failed: {error}"));
            }
            if let Err(error) = tree_backup.rollback() {
                rollback_failures.push(format!("tree rollback failed: {error}"));
            }
            if rollback_failures.is_empty() {
                return Err(MgError::Other(format!(
                    "{script_err} (lockfile rolled back to pre-install state; re-run with --ignore-scripts to skip lifecycle scripts)"
                )));
            }
            return Err(MgError::Other(format!(
                "{script_err} (ROLLBACK ALSO FAILED — project may be inconsistent, delete node_modules + mgc.lock and re-run install: {})",
                rollback_failures.join("; ")
            )));
        }
    }

    // Scripts are skipped or succeeded: keep the matching new tree and lock.
    // Script được bỏ qua hoặc chạy xong: giữ cây và lock mới đồng nhất.
    tree_backup.commit();
    state.profile.mark("lifecycle_scripts", state.start);
    Ok(())
}
