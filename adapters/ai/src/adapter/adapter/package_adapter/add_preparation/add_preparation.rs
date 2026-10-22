//! Prepares native AI package additions through the embedded Python engine.
//! Chuẩn bị thêm package AI native qua Python engine nhúng.

use crate::adapter::AiAdapter;
use mgc_types::PackageAdapter;
use mgc_types::adapter::{AddOptions, PreparedAdd};
use mgc_types::{MgResult, PackageName, VersionRange};
use std::path::Path;

/// PyPI lane: delegate to the embedded python engine — resolve-first
/// (same contract as lib), so no fake-add ever persists. Non-lane:
/// fail-closed until MGC owns the Python manifest.
/// (Lane PyPI: chuẩn bị qua engine nhúng, chưa ghi file. Không lane:
/// fail-closed tới khi MGC sở hữu manifest Python.)
pub(in crate::adapter::implementation) async fn prepare_add(
    adapter: &AiAdapter,
    project_root: &Path,
    name: &PackageName,
    range: Option<&VersionRange>,
    opts: AddOptions,
) -> MgResult<PreparedAdd> {
    match &adapter.python_lane {
        Some(py) => py.prepare_add(project_root, name, range, opts).await,
        None => Err(mgc_types::capabilities::unsupported_capability(
            "ai",
            "prepare-add",
            "no pyproject.toml — native AI dependency edits require an MGC-owned pyproject; foreign lockfiles are unsupported",
        )),
    }
}
