//! Web delegation or Cloud infrastructure-manifest fallback — delegate Web hoặc fallback manifest hạ tầng Cloud.

use mgc_types::PackageAdapter;
use std::path::Path;

use mgc_types::{Ecosystem, Manifest, MgResult};

use super::super::super::CloudAdapter;

pub(in crate::adapter::package_adapter) async fn parse_manifest(
    adapter: &CloudAdapter,
    project_root: &Path,
) -> MgResult<Manifest> {
    if let Some(web) = &adapter.web {
        return web.parse_manifest(project_root).await;
    }
    let name = project_root
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "infra".to_string());
    Ok(Manifest::new(&name, Ecosystem::Cloud))
}
