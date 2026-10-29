//! Finds the project, applies shared mutation gates and dispatches store commands.
//! Tìm project, áp gate mutation dùng chung và điều phối lệnh store.

use super::super::command_types::StoreCmd;

pub async fn run(cmd: StoreCmd) -> anyhow::Result<()> {
    // Measurement is read-only and applies to any core: it must not require a
    // web project, a project root, or the web store owner gate.
    // (Đo đạc chỉ đọc và áp dụng cho mọi core: không cần project web, không cần
    // project root, không qua gate web store owner.)
    if let StoreCmd::Stats { json, root } = &cmd {
        let store_root = match root {
            Some(path) => std::path::PathBuf::from(path),
            None => super::super::store_paths::store_cache_root_for(&std::env::current_dir()?),
        };
        let stats = crate::commands::core::store::collect(&store_root)?;
        crate::commands::core::store::print(&stats, *json);
        return Ok(());
    }

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
        // Handled before the project lookup above; measurement never mutates.
        // (Đã xử lý trước bước tìm project; đo đạc không bao giờ thay đổi.)
        StoreCmd::Stats { .. } => Ok(()),
        StoreCmd::Prune { dry_run, json } => {
            super::super::prune::run_prune(&project_root, dry_run, json)
        }
        StoreCmd::Reclaim { apply, yes, json } => {
            super::super::reclaim::run_reclaim(&project_root, apply, yes, json)
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
