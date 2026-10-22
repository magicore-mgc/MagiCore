//! Language-specific lockfile capability.
//! Capability lockfile theo ngôn ngữ.

use crate::adapter::AppAdapter;
use crate::language::AppLanguage;
use async_trait::async_trait;
use mgc_types::capabilities::LockfileProvider;
use mgc_types::{Manifest, MgResult};
use std::path::Path;

#[async_trait]
impl LockfileProvider for AppAdapter {
    fn probe_lockfile_provider(&self) -> MgResult<()> {
        match self.language {
            AppLanguage::Flutter | AppLanguage::Swift => Ok(()),
            AppLanguage::ReactNative => Err(mgc_types::capabilities::unsupported_capability(
                "app",
                "lockfile_provider",
                "React Native install is not available until all ecosystem lock entries can be committed atomically",
            )),
            AppLanguage::Kotlin
            | AppLanguage::ObjC
            | AppLanguage::Multi
            | AppLanguage::Maui
            | AppLanguage::Tauri => Err(mgc_types::capabilities::unsupported_capability(
                "app",
                "lockfile_provider",
                "this ecosystem has no MagiCore-owned lockfile writer",
            )),
        }
    }

    async fn write_manifest(&self, project_root: &Path, manifest: &Manifest) -> MgResult<()> {
        if !matches!(self.language, AppLanguage::Flutter | AppLanguage::Swift) {
            return Err(mgc_types::capabilities::unsupported_capability(
                "app",
                "write_manifest",
                "this app ecosystem has no safe MagiCore-owned manifest writer",
            ));
        }
        crate::manifest::write_manifest(self.language, project_root, manifest)
    }
}
