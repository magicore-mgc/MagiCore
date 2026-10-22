//! Build the lifecycle child PATH and resolve its bounded execution timeout.
//! Tạo PATH cho tiến trình lifecycle và xác định timeout thực thi có giới hạn.

use super::error::{LifecycleError, LifecycleResult};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::Duration;

const DEFAULT_LIFECYCLE_TIMEOUT_SECS: u64 = 300;
const LIFECYCLE_TIMEOUT_ENV: &str = "MGC_LIFECYCLE_TIMEOUT_SECS";

pub(super) fn lifecycle_timeout() -> Duration {
    std::env::var(LIFECYCLE_TIMEOUT_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|secs| *secs > 0)
        .map(Duration::from_secs)
        .unwrap_or(Duration::from_secs(DEFAULT_LIFECYCLE_TIMEOUT_SECS))
}

pub(super) fn lifecycle_path_env(project_root: &Path) -> LifecycleResult<OsString> {
    let node_modules_bin = project_root.join("node_modules").join(".bin");
    let current_paths: Vec<PathBuf> = std::env::var_os("PATH")
        .map(|paths| std::env::split_paths(&paths).collect())
        .unwrap_or_default();

    if !node_modules_bin.exists() {
        return std::env::join_paths(current_paths).map_err(LifecycleError::PathBuild);
    }

    let mut paths = Vec::with_capacity(current_paths.len() + 1);
    paths.push(node_modules_bin);
    paths.extend(current_paths);
    std::env::join_paths(paths).map_err(LifecycleError::PathBuild)
}
