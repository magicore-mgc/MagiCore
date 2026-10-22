//! PackageAdapter trait forwarding implementation for LibAdapter.
//! Triển khai chuyển tiếp trait PackageAdapter cho LibAdapter.

use super::{
    arm_age_gate_for, list_installed, manifest_identity, parse_manifest as parse_lib_manifest,
    prepare_add, select_capabilities, set_dedupe_pref, set_existing_versions,
    supports_age_gate_for,
};
use crate::adapter::types::LibAdapter;
use async_trait::async_trait;
use mgc_types::Capability;
use mgc_types::adapter::{AddOptions, InstalledPackage, PackageAdapter, PreparedAdd};
use mgc_types::{Manifest, MgResult, PackageName, VersionRange};
use std::path::Path;

#[async_trait]
impl PackageAdapter for LibAdapter {
    fn capabilities(&self) -> &'static [Capability] {
        select_capabilities(self)
    }

    fn manifest_identity(&self) -> Option<mgc_types::ManifestIdentity> {
        manifest_identity(self)
    }

    fn arm_age_gate_for(&self, project_root: &Path) -> MgResult<()> {
        arm_age_gate_for(self, project_root)
    }

    fn supports_age_gate_for(&self, _project_root: &Path) -> MgResult<bool> {
        supports_age_gate_for(self, _project_root)
    }

    fn manifest_owned(&self) -> bool {
        self.python_native_owned()
    }

    fn supports_native_update(&self) -> bool {
        self.resolve_first_lane().is_ok()
    }

    async fn prepare_add(
        &self,
        project_root: &Path,
        name: &PackageName,
        range: Option<&VersionRange>,
        opts: AddOptions,
    ) -> MgResult<PreparedAdd> {
        prepare_add(self, project_root, name, range, opts).await
    }

    async fn parse_manifest(&self, project_root: &Path) -> MgResult<Manifest> {
        parse_lib_manifest(self, project_root).await
    }

    async fn list(&self, project_root: &Path) -> MgResult<Vec<InstalledPackage>> {
        list_installed(self, project_root).await
    }

    fn set_dedupe_pref(&self, enabled: bool) {
        set_dedupe_pref(self, enabled);
    }

    fn set_existing_versions(&self, versions: std::collections::HashMap<String, String>) {
        set_existing_versions(self, versions);
    }
}
