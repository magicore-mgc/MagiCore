//! MGC install arguments and inherited caller policy for the ephemeral project.
//! Argument cài MGC và policy caller được kế thừa cho project tạm.

use anyhow::Result;
use std::path::Path;

pub(in crate::commands::package_runtime) const PACKAGE_RUNNER_PROJECT_NAME: &str =
    "magicore-package-runner";
pub(in crate::commands::package_runtime) const PACKAGE_RUNNER_PROJECT_VERSION: &str = "1.0.0";

pub(crate) fn registry_install_args(allow_scripts: bool) -> Vec<String> {
    let mut args = ["install", "--core", "web"]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    args.push(if allow_scripts {
        "--allow-scripts".to_string()
    } else {
        "--ignore-scripts".to_string()
    });
    args
}

// Preserve caller install policy without importing framework or compat execution settings.
// Giữ policy cài đặt của caller; project tạm vẫn thuộc web và không tự mở compat.
pub(crate) fn registry_project_config(cwd: &Path) -> Result<mgc_config::project::ProjectConfig> {
    let mut config = mgc_config::project::ProjectConfig::new(PACKAGE_RUNNER_PROJECT_NAME, "web");
    config.version = PACKAGE_RUNNER_PROJECT_VERSION.to_string();
    if let Some(caller) = mgc_config::project::ProjectConfig::load(cwd)
        .map_err(|_| super::registry_package_invalid_caller_policy())?
    {
        config.registries = caller.registries;
        config.security = caller.security;
        config.lock = caller.lock;
        config.trust = caller.trust;
    }
    Ok(config)
}
#[cfg(test)]
#[path = "test/install_policy.rs"]
mod tests;
