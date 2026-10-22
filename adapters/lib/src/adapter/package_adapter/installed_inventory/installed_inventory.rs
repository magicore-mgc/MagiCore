//! Verified installed-package inventory for supported Lib lanes.
//! Inventory package đã cài có thể xác minh cho lane Lib được hỗ trợ.

use super::super::parse_manifest;
use crate::adapter::types::LibAdapter;
use crate::language::LibLanguage;
use mgc_types::MgResult;
use mgc_types::PackageAdapter;
use mgc_types::adapter::InstalledPackage;
use std::path::Path;

pub(in crate::adapter::package_adapter) async fn list_installed(
    adapter: &LibAdapter,
    project_root: &Path,
) -> MgResult<Vec<InstalledPackage>> {
    adapter.require_python_native_at(project_root, "list")?;
    if let Some(web) = &adapter.web {
        return web.list(project_root).await;
    }
    if adapter.language != LibLanguage::Python {
        return Err(mgc_types::capabilities::unsupported_capability(
            "lib",
            "list installed packages",
            "this language lane does not yet maintain a verified installed-artifact inventory; manifest declarations and lock pins are not proof that packages are installed",
        ));
    }
    let manifest = parse_manifest(adapter, project_root).await?;
    crate::install::native_python_installed_packages(project_root, &manifest)
}
