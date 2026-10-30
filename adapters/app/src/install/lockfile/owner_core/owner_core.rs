//! Resolve the App lock owner's core / Xác định core sở hữu lock App.

use super::super::error::AppLockfileError;
use std::path::Path;

pub(in crate::install::lockfile) fn app_lock_owner_core(
    project_root: &Path,
) -> Result<String, AppLockfileError> {
    let marker =
        mgc_config::project::ProjectConfig::read_core_marker(project_root).map_err(|error| {
            AppLockfileError::caused(
                "MGC_APP_LOCK_OWNER_MARKER",
                "read lock owner marker",
                format!("cannot determine lock owner core: {error}"),
                error,
            )
        })?;
    let owner = match marker {
        Some(core) => core,
        None => match mgc_config::project::ProjectConfig::load(project_root).map_err(|error| {
            AppLockfileError::caused(
                "MGC_APP_LOCK_OWNER_CONFIG",
                "read lock owner config",
                format!("cannot read project core owner: {error}"),
                error,
            )
        })? {
            Some(config) => canonical_core_name(&config.ecosystem),
            None => "app".to_string(),
        },
    };
    if !mgc_config::project::ProjectConfig::KNOWN_CORES.contains(&owner.as_str()) {
        return Err(AppLockfileError::message(
            "MGC_APP_LOCK_UNKNOWN_OWNER",
            "validate lock owner core",
            format!("refusing to write app lock entries for unknown core owner '{owner}'"),
        ));
    }
    Ok(owner)
}

fn canonical_core_name(value: &str) -> String {
    // One alias table for the whole workspace (`mgc_types::Ecosystem`); the
    // local `if core == "cloud"` copy was a compensating layer for H-1.
    // (Một bảng alias duy nhất; bản copy `if core == "cloud"` là lớp bù cho H-1.)
    let core = value.trim().to_ascii_lowercase();
    mgc_types::Ecosystem::canonical_core_name(&core)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
