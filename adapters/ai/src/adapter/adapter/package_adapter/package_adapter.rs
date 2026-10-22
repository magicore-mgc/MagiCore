//! PackageAdapter trait forwarding implementation for AiAdapter.
//! Triển khai chuyển tiếp trait PackageAdapter cho AiAdapter.

use super::super::AiAdapter;
use super::{
    arm_age_gate_for, list_installed, manifest_identity, parse_manifest_for_adapter, prepare_add,
    select_capabilities, supports_age_gate_for, supports_native_update,
};
use async_trait::async_trait;
use mgc_types::adapter::{AddOptions, InstalledPackage, PackageAdapter, PreparedAdd};
use mgc_types::capabilities::Capability;
use mgc_types::{Manifest, MgResult, PackageName, VersionRange};
use std::path::Path;

#[async_trait]
impl PackageAdapter for AiAdapter {
    fn capabilities(&self) -> &'static [Capability] {
        select_capabilities(self)
    }

    fn supports_native_update(&self) -> bool {
        supports_native_update(self)
    }

    fn manifest_identity(&self) -> Option<mgc_types::ManifestIdentity> {
        manifest_identity(self)
    }

    async fn parse_manifest(&self, project_root: &Path) -> MgResult<Manifest> {
        parse_manifest_for_adapter(self, project_root)
    }

    async fn list(&self, project_root: &Path) -> MgResult<Vec<InstalledPackage>> {
        list_installed(self, project_root).await
    }

    /// PyPI lane: delegate to the embedded python engine — resolve-first
    /// (same contract as lib), so no fake-add ever persists. Non-lane:
    /// fail-closed until MGC owns the Python manifest.
    /// (Lane PyPI: chuẩn bị qua engine nhúng, chưa ghi file. Không lane:
    /// fail-closed tới khi MGC sở hữu manifest Python.)
    async fn prepare_add(
        &self,
        project_root: &Path,
        name: &PackageName,
        range: Option<&VersionRange>,
        opts: AddOptions,
    ) -> MgResult<PreparedAdd> {
        prepare_add(self, project_root, name, range, opts).await
    }

    /// P0/F6: the embedded engine's gate arms from the same project root.
    /// (Cổng tuổi của engine nhúng nạp theo cùng project root.)
    fn arm_age_gate_for(&self, project_root: &Path) -> MgResult<()> {
        arm_age_gate_for(self, project_root)
    }

    fn supports_age_gate_for(&self, project_root: &Path) -> MgResult<bool> {
        supports_age_gate_for(self, project_root)
    }
}
