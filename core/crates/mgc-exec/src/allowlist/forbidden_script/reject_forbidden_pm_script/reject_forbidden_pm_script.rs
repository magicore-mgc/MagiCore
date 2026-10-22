use crate::allowlist::find_forbidden_tool_in_script;
use anyhow::{Result, bail};

/// Reject scripts that attempt to delegate back into external PMs.
/// Chặn script vòng ngược qua npm/pnpm/yarn/bun để giữ MagiCore độc lập.
pub fn reject_forbidden_pm_script(script: &str) -> Result<()> {
    if let Some(tool) = find_forbidden_tool_in_script(script) {
        bail!(
            "script delegates to forbidden package manager '{tool}'; use MagiCore-native install/run commands instead"
        );
    }
    Ok(())
}
