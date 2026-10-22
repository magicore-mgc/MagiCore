//! Stable PackageAdapter trait boundary; behavior is delegated to nearby owners.
//! Ranh giới trait PackageAdapter ổn định; hành vi chuyển đến owner gần kề.

use async_trait::async_trait;
use std::path::Path;

use mgc_types::adapter::{AddOptions, InstalledPackage, PackageAdapter, PreparedAdd};
use mgc_types::capabilities::Capability;
use mgc_types::{Manifest, MgResult, PackageName, VersionRange};

use super::super::CloudAdapter;
use super::{listing, manifest, project_metadata, web_delegation};

#[async_trait]
impl PackageAdapter for CloudAdapter {
    fn capabilities(&self) -> &'static [Capability] {
        project_metadata::capabilities(self)
    }

    fn manifest_identity(&self) -> Option<mgc_types::ManifestIdentity> {
        project_metadata::manifest_identity(self)
    }

    async fn prepare_add(
        &self,
        project_root: &Path,
        name: &PackageName,
        range: Option<&VersionRange>,
        opts: AddOptions,
    ) -> MgResult<PreparedAdd> {
        web_delegation::prepare_add(self, project_root, name, range, opts).await
    }

    /// P0/F6: forward to the embedded web engine (TS delegate resolves
    /// through it — its gate must arm from the same project).
    /// (Chuyển cho web engine nhúng.)
    fn arm_age_gate_for(&self, project_root: &std::path::Path) -> MgResult<()> {
        web_delegation::arm_age_gate_for(self, project_root)
    }

    fn supports_age_gate_for(&self, _project_root: &std::path::Path) -> MgResult<bool> {
        web_delegation::supports_age_gate_for(self)
    }

    fn set_dedupe_pref(&self, enabled: bool) {
        web_delegation::set_dedupe_pref(self, enabled);
    }

    fn set_existing_versions(&self, versions: std::collections::HashMap<String, String>) {
        web_delegation::set_existing_versions(self, versions);
    }

    async fn parse_manifest(&self, project_root: &Path) -> MgResult<Manifest> {
        manifest::parse_manifest(self, project_root).await
    }

    async fn list(&self, project_root: &Path) -> MgResult<Vec<InstalledPackage>> {
        listing::list(self, project_root).await
    }
}
