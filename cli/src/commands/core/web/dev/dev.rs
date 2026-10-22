//! Performs Web dev preflight and starts the selected single/multi-target runner.
//! Thực hiện dev preflight Web và chạy single/multi-target runner đã chọn.

use crate::commands::freshness::check_profile_gate;
use anyhow::Result;
use std::path::Path;

use super::super::{check_dev_freshness, run_multi_dev_targets, run_single_dev_target};
use super::target_collection::dev_targets;

pub async fn dev_at_root(
    project_root: &Path,
    host: Option<String>,
    port: Option<u16>,
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
) -> Result<()> {
    // Refresh policy distribution once, resolve every launch target, then
    // finish freshness and profile pre-flight for each target before any
    // lifecycle hook or dev process can spawn.
    // (Refresh policy một lần, resolve toàn bộ target, rồi hoàn tất freshness
    // và profile pre-flight từng target trước khi spawn hook hay dev process.)
    crate::commands::policy::refresh_url_manifest_if_configured(project_root).await;
    let targets = dev_targets(project_root, host, port)?;
    let profile_store = crate::commands::freshness::default_profile_store_dir();
    for target in &targets {
        check_dev_freshness(&target.dir)?;
        // Pre-flight row 3: known-mutation lanes refuse upfront (never mid-run).
        // (Hàng 3: lane mutation đã biết từ chối upfront, không kill giữa run.)
        check_profile_gate(&target.dir, &profile_store, "dev", compat_runner)?;
    }
    if targets.len() == 1 {
        return run_single_dev_target(&targets[0], compat, compat_runner).await;
    }

    run_multi_dev_targets(&targets, compat, compat_runner).await
}
