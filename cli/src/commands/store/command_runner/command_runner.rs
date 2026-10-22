//! Finds the project, applies shared mutation gates and dispatches store commands.
//! Tìm project, áp gate mutation dùng chung và điều phối lệnh store.

use super::super::command_types::StoreCmd;

pub async fn run(cmd: StoreCmd) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = mgc_config::project::ProjectConfig::find_project_root(&cwd)
        .ok_or_else(crate::error::project_root_missing)?;

    if let StoreCmd::Doctor {
        repair,
        core,
        framework,
        json,
    } = &cmd
    {
        return super::super::doctor_dispatch::run_store_doctor(
            &project_root,
            *repair,
            core,
            framework.as_deref(),
            *json,
        );
    }

    super::super::project_ownership::ensure_web_store_owner(&project_root)?;
    let native = crate::commands::compat::CompatMode::Native;
    crate::commands::dep_gate::gate(
        &crate::commands::dep_gate::DepContext::new(
            "web",
            Some(crate::commands::dep_gate::eco::JS),
            None,
            None,
            crate::commands::dep_gate::DepOp::Gc,
        ),
        None,
        &native,
        None,
    )?;

    match cmd {
        StoreCmd::Prune { dry_run, json } => {
            super::super::prune::run_prune(&project_root, dry_run, json)
        }
        StoreCmd::Status => super::super::store_status::run_status(&project_root),
        StoreCmd::Backup { path } => super::super::backup::run_backup(&project_root, path),
        StoreCmd::Restore { path } => super::super::restore::run_restore(&project_root, path),
        StoreCmd::Doctor {
            repair,
            core,
            framework,
            json,
        } => super::super::doctor_dispatch::run_store_doctor(
            &project_root,
            repair,
            &core,
            framework.as_deref(),
            json,
        ),
    }
}
