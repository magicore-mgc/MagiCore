//! PackageAdapter trait forwarding implementation for AppAdapter.
//! Triển khai chuyển tiếp trait PackageAdapter cho AppAdapter.

use super::{
    list_installed, lock_replay_manifest, manifest_identity, parse_manifest_for_adapter,
    prepare_add, resolves_implicit_dependencies, select_capabilities, supports_native_update,
};
use crate::adapter::AppAdapter;
use async_trait::async_trait;
use mgc_types::adapter::{AddOptions, InstalledPackage, PackageAdapter, PreparedAdd};
use mgc_types::capabilities::Capability;
use mgc_types::{Manifest, MgResult, PackageName, VersionRange};
use std::path::Path;

#[async_trait]
impl PackageAdapter for AppAdapter {
    fn lock_replay_manifest(&self, project_root: &Path, manifest: &Manifest) -> MgResult<Manifest> {
        lock_replay_manifest(self, project_root, manifest)
    }

    fn resolves_implicit_dependencies(&self, project_root: &Path) -> MgResult<bool> {
        resolves_implicit_dependencies(self, project_root)
    }

    fn manifest_identity(&self) -> Option<mgc_types::ManifestIdentity> {
        manifest_identity(self)
    }

    fn capabilities(&self) -> &'static [Capability] {
        select_capabilities(self)
    }

    async fn parse_manifest(&self, project_root: &Path) -> MgResult<Manifest> {
        parse_manifest_for_adapter(self, project_root)
    }

    fn supports_native_update(&self) -> bool {
        supports_native_update(self)
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

    async fn list(&self, project_root: &Path) -> MgResult<Vec<InstalledPackage>> {
        list_installed(self, project_root).await
    }
}
