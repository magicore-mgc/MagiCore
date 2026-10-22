//! List packages only on the supported Web-backed manifest lane — chỉ list ở lane manifest Web được hỗ trợ.

use mgc_types::PackageAdapter;
use std::path::Path;

use mgc_types::adapter::InstalledPackage;
use mgc_types::{MgError, MgResult};

use super::super::super::CloudAdapter;

pub(in crate::adapter::package_adapter) async fn list(
    adapter: &CloudAdapter,
    project_root: &Path,
) -> MgResult<Vec<InstalledPackage>> {
    if let Some(web) = &adapter.web {
        if project_root.join("package.json").is_file() {
            return web.list(project_root).await;
        }
        return Err(MgError::Unsupported {
            core: "cloud",
            capability: "list",
            guidance:
                "cloud dependency listing currently supports CDK/Pulumi projects with a package.json manifest only"
                    .to_string(),
        });
    }
    Err(MgError::Unsupported {
        core: "cloud",
        capability: "list",
        guidance: format!(
            "{} does not expose a dependency manifest that MagiCore can list",
            adapter.cloud_type.as_str()
        ),
    })
}
