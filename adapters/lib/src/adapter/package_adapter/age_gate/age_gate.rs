//! Project-specific package-age policy hooks for LibAdapter.
//! Hook chính sách tuổi package theo project cho LibAdapter.

use crate::adapter::types::LibAdapter;
use crate::language::LibLanguage;
use mgc_types::MgResult;
use std::path::Path;

pub(in crate::adapter::package_adapter) fn arm_age_gate_for(
    adapter: &LibAdapter,
    project_root: &Path,
) -> MgResult<()> {
    if let Some(web) = &adapter.web {
        web.arm_age_gate_for_core(project_root, adapter.security_core)?;
    } else {
        adapter.ensure_age_gate_supported(project_root)?;
    }
    if adapter.language == LibLanguage::Python {
        // Validate this operation's project config before resolving.
        // Xác thực config của project cho operation này trước khi resolve.
        adapter.pypi_protocol_for(project_root)?;
    }
    Ok(())
}

pub(in crate::adapter::package_adapter) fn supports_age_gate_for(
    adapter: &LibAdapter,
    _project_root: &Path,
) -> MgResult<bool> {
    Ok(adapter.language == LibLanguage::Python || adapter.web.is_some())
}
