//! Delegates package-age gate operations to the native Python lane.
//! Ủy quyền thao tác cổng tuổi package cho lane Python native.

use crate::adapter::AiAdapter;
use mgc_types::MgResult;
use mgc_types::PackageAdapter;
use std::path::Path;

/// P0/F6: the embedded engine's gate arms from the same project root.
/// (Cổng tuổi của engine nhúng nạp theo cùng project root.)
pub(in crate::adapter::implementation) fn arm_age_gate_for(
    adapter: &AiAdapter,
    project_root: &Path,
) -> MgResult<()> {
    if let Some(py) = &adapter.python_lane {
        return py.arm_age_gate_for(project_root);
    }
    Ok(())
}

pub(in crate::adapter::implementation) fn supports_age_gate_for(
    adapter: &AiAdapter,
    project_root: &Path,
) -> MgResult<bool> {
    match &adapter.python_lane {
        Some(py) => py.supports_age_gate_for(project_root),
        None => Ok(false),
    }
}
