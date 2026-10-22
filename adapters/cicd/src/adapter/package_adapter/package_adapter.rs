//! Keep hand-owned workflow files outside package-install semantics.
//! Giữ workflow do người dùng quản lý ngoài semantics cài package.

use super::super::adapter::CicdAdapter;
use async_trait::async_trait;
use mgc_types::adapter::{InstalledPackage, PackageAdapter};
use mgc_types::capabilities::Capability;
use mgc_types::{Ecosystem, Manifest, MgResult};
use std::path::Path;

#[async_trait]
impl PackageAdapter for CicdAdapter {
    fn capabilities(&self) -> &'static [Capability] {
        Self::CAPABILITIES
    }

    async fn parse_manifest(&self, project_root: &Path) -> MgResult<Manifest> {
        let name = project_root
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "ci".to_string());
        Ok(Manifest::new(&name, Ecosystem::Cicd))
    }

    async fn list(&self, project_root: &Path) -> MgResult<Vec<InstalledPackage>> {
        let _ = project_root;
        Err(mgc_types::MgError::Unsupported {
            core: "cicd",
            capability: "list",
            guidance: concat!(
                "CI/CD workflow files are not package manifests; ",
                "use workflow/provider inspection instead"
            )
            .to_string(),
        })
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
