use crate::allowlist::{ExecutionScope, FORBIDDEN_TOOLS};
use anyhow::{Result, bail};

/// Deny package-manager executables before any allowlist authorization.
/// Chặn executable package manager trước mọi bước cấp quyền allowlist.
pub(in crate::allowlist::tool_check) fn reject_forbidden_pm_tool(
    name: &str,
    normalized: &str,
    scope: ExecutionScope,
) -> Result<()> {
    if FORBIDDEN_TOOLS.contains(&normalized) {
        bail!(
            "tool '{name}' is forbidden in {:?} scope; dependency operations must be owned by MagiCore",
            scope
        );
    }
    Ok(())
}
