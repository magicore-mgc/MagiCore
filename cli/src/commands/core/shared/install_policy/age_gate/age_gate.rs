//! Apply minimum-release-age policy using the selected core adapter.
//! Áp policy minimum-release-age theo adapter core được chọn.

use super::error::AgeGateError;
use anyhow::Result;
use mgc_types::adapter::PackageAdapter;
use std::path::Path;

pub(crate) fn age_gate_lock_action(
    core: &str,
    age_gate_active: bool,
    age_gate_supported: bool,
    locked_graph_available: bool,
    frozen: bool,
) -> Result<bool> {
    if !age_gate_active {
        return Ok(false);
    }
    if !age_gate_supported {
        return Err(AgeGateError::UnsupportedResolver {
            core: core.to_string(),
        }
        .into());
    }
    if locked_graph_available && frozen {
        return Err(AgeGateError::FrozenLockMissingPublicationTimestamps.into());
    }
    Ok(locked_graph_available)
}

pub(crate) fn refresh_locked_graph_for_age_gate(
    adapter: &dyn PackageAdapter,
    project_root: &Path,
    locked_graph_available: bool,
    frozen: bool,
) -> Result<bool> {
    let Some(security) = mgc_config::project::load_security_config(project_root)? else {
        return Ok(false);
    };
    let active = security
        .min_age_for_ecosystem(adapter.core_id())
        .is_some_and(|cutoff| cutoff > 0);
    let supported = if active {
        adapter.supports_age_gate_for(project_root)?
    } else {
        false
    };
    age_gate_lock_action(
        adapter.core_id(),
        active,
        supported,
        locked_graph_available,
        frozen,
    )
}
