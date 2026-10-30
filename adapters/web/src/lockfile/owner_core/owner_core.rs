//! `owner_core.rs` — project lock-owner marker and core canonicalization.
//! marker owner lock theo project và chuẩn hóa tên core

use super::super::LockfileError;
use mgc_types::MgResult;
use std::path::Path;

/// Resolve the caller core from the project signature so embedded web
/// engines (Cloud/CDK, App/React Native) do not relabel or prune another
/// core's package entries. Missing metadata retains the historical Web
/// default; malformed metadata fails closed.
/// (Lấy core caller từ chữ ký project để engine web nhúng không ghi đè core khác.)
pub(crate) fn web_lock_owner_core(project_root: &Path) -> MgResult<String> {
    let marker =
        mgc_config::project::ProjectConfig::read_core_marker(project_root).map_err(|error| {
            LockfileError::Ownership(format!("cannot determine lock owner core: {error}"))
        })?;
    let owner = match marker {
        Some(core) => core,
        None => match mgc_config::project::ProjectConfig::load(project_root).map_err(|error| {
            LockfileError::Ownership(format!("cannot read project core owner: {error}"))
        })? {
            Some(config) => canonical_core_name(&config.ecosystem),
            None => "web".to_string(),
        },
    };
    if !mgc_config::project::ProjectConfig::KNOWN_CORES.contains(&owner.as_str()) {
        return Err(LockfileError::Ownership(format!(
            "refusing to write lock entries for unknown core owner '{owner}'"
        ))
        .into());
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
