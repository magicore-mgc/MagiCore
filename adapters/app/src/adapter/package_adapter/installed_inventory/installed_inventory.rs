//! Refuse to infer installed versions from App manifest declarations.
//! Không suy luận phiên bản đã cài từ khai báo trong manifest App.

use super::super::parse_manifest_for_adapter;
use crate::adapter::AppAdapter;
use mgc_types::adapter::InstalledPackage;
use mgc_types::{MgError, MgResult};
use std::path::Path;

pub(in crate::adapter::package_adapter) async fn list_installed(
    adapter: &AppAdapter,
    project_root: &Path,
) -> MgResult<Vec<InstalledPackage>> {
    let manifest = parse_manifest_for_adapter(adapter, project_root)?;
    if manifest.all_dependencies().next().is_none() {
        return Ok(Vec::new());
    }
    Err(MgError::Unsupported {
        core: "app",
        capability: "list",
        guidance: "the app adapter does not yet reconcile resolved locks with installed files; refusing to label declared/ranged dependencies as installed versions".to_string(),
    })
}
