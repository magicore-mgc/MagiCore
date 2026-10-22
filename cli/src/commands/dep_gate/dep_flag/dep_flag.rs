//! Reject compatibility execution for dependency operations — chặn compat trong dependency.
//! Explicit flags and the inherited environment setting are refused before provider selection.
//! Cờ tường minh và biến môi trường bị từ chối trước khi chọn provider.

use crate::commands::compat::CompatMode;
use crate::commands::dep_gate::DepGateError;
use anyhow::Result;

/// Dependency operations are native-only: any explicit compat value is
/// rejected before a command can select a provider package manager.
/// Dependency chỉ chạy native; mọi compat value bị chặn trước khi chọn PM.
pub fn from_dep_flag(flag: Option<&str>) -> Result<CompatMode> {
    let explicit = flag
        .map(str::to_owned)
        .or_else(|| std::env::var("MGC_COMPAT_RUNTIME").ok());
    if let Some(value) = explicit {
        return Err(DepGateError::compat_disabled(&value).into());
    }
    Ok(CompatMode::Native)
}
