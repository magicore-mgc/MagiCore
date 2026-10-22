//! Coordinates the `mgc trust pending` report without owning scan internals.
//! Điều phối báo cáo `mgc trust pending`, không triển khai chi tiết quét.

use anyhow::{Context, Result};
use mgc_store::Layout;
use std::env;

use super::{
    acquire_scan_lock, load_db_policies, load_file_policy, render_report, scan_scripted_packages,
};

pub fn execute() -> Result<()> {
    let project_root = env::current_dir().context("failed to get current directory")?;
    // Keep the package tree and project trust policy stable for the whole
    // report; never publish a mixed view while install/remove is in flight.
    // (Giữ ổn định cây package và policy trong suốt lần quét.)
    let _scan_lock = acquire_scan_lock(
        &project_root,
        crate::commands::core::shared::writer_lock_timeout(&project_root),
    )?;
    let node_modules = project_root.join("node_modules");
    if !node_modules.exists() {
        anyhow::bail!(
            "no node_modules in {} — run install first",
            project_root.display()
        );
    }

    // Machine-local DB policies (name[@version] -> approved|denied).
    // (Policy DB máy local.)
    let cache_root = project_root.join(".magicore").join("cache").join("web");
    let layout = Layout::new(cache_root);
    let db_policies = load_db_policies(&layout.db_path())?;

    // Committed file policy (mgc.toml [scripts]) — same merge rule as the
    // install gate: deny wins from either source, then allow. A broken
    // table aborts this report instead of silently dropping a deny.
    // (Policy file commit được — cùng quy tắc hợp nhất với gate install.)
    let file_policy = load_file_policy(&project_root)?;

    // Scan installed packages for lifecycle scripts RECURSIVELY — the
    // install runner executes nested node_modules too, so a shallow
    // top-level-only scan would hide scripted transitive deps from
    // review. Symlink cycles (hoisted links) are cut via a canonicalized
    // visited set.
    // (Quét đệ quy khớp runner install — symlink cycle bị cắt.)
    let scripted = scan_scripted_packages(&node_modules)?;

    render_report(scripted, &db_policies, file_policy.as_ref());
    Ok(())
}
