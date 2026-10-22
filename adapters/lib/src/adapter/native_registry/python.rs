//! Construct the project-configured PyPI registry protocol.
//! Dựng protocol PyPI theo cấu hình project.

use crate::adapter::types::LibAdapter;
use mgc_resolver::protocols::RegistryProtocol;
use mgc_types::MgResult;
use std::path::Path;

pub(super) fn create(
    adapter: &LibAdapter,
    project_root: &Path,
) -> MgResult<Box<dyn RegistryProtocol>> {
    Ok(Box::new(adapter.pypi_protocol_for(project_root)?))
}
