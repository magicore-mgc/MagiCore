//! Keep the hardware adapter's package graph deliberately unsupported.
//! Giữ package graph của adapter hardware ở trạng thái unsupported.

use super::super::adapter::HardwareAdapter;
use async_trait::async_trait;
use mgc_types::adapter::{InstalledPackage, PackageAdapter};
use mgc_types::capabilities::Capability;
use mgc_types::{Ecosystem, Manifest, MgResult};
use std::path::Path;

#[async_trait]
impl PackageAdapter for HardwareAdapter {
    fn capabilities(&self) -> &'static [Capability] {
        Self::CAPABILITIES
    }

    async fn parse_manifest(&self, project_root: &Path) -> MgResult<Manifest> {
        let name = project_root
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| "hardware".to_string());
        Ok(Manifest::new(&name, Ecosystem::Hardware))
    }

    async fn list(&self, project_root: &Path) -> MgResult<Vec<InstalledPackage>> {
        let _ = project_root;
        Err(mgc_types::MgError::Unsupported {
            core: "hardware",
            capability: "list",
            guidance: concat!(
                "optimizer/bench folders are MagiCore templates, not installed packages; ",
                "use `mgc optimizer` or inspect the project templates directly"
            )
            .to_string(),
        })
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
