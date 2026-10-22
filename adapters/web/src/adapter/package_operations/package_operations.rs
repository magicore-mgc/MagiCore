//! `package_operations.rs` — Package-adapter operations / thao tác package adapter.
use super::super::WebAdapter;
use crate::audit::run_audit_fix;
use crate::manifest::parse_manifest;
use async_trait::async_trait;
use mgc_types::PackageAdapter;
use mgc_types::{
    Manifest, MgResult, PackageId, PackageName, Version, VersionRange,
    adapter::{AddOptions, InstalledPackage},
    capabilities::{Capability, DependencyResolver},
};
use std::path::Path;

#[async_trait]
impl PackageAdapter for WebAdapter {
    fn capabilities(&self) -> &'static [Capability] {
        Self::CAPABILITIES
    }

    /// MGC owns the npm resolver and package.json writer for this adapter.
    /// MGC sở hữu resolver npm và writer package.json của adapter này.
    fn supports_native_update(&self) -> bool {
        true
    }

    fn manifest_identity(&self) -> Option<mgc_types::ManifestIdentity> {
        Some(mgc_types::ManifestIdentity {
            core: "web".to_string(),
            language: "js".to_string(),
            format: "package.json".to_string(),
            relpath: "package.json".to_string(),
        })
    }

    /// P0/F6: arm per operation from that operation's project root
    /// (overwrites — no first-writer-wins across projects).
    /// (Nạp cổng tuổi theo từng operation từ root project của nó.)
    fn arm_age_gate_for(&self, project_root: &std::path::Path) -> mgc_types::MgResult<()> {
        self.arm_age_gate_for_core(project_root, "web")
    }

    fn supports_age_gate_for(&self, _project_root: &std::path::Path) -> mgc_types::MgResult<bool> {
        Ok(true)
    }

    fn set_dedupe_pref(&self, enabled: bool) {
        self.dedupe_pref
            .store(enabled, std::sync::atomic::Ordering::Relaxed);
        self.resolver.set_dedupe_pref(if enabled {
            mgc_resolver::solver::DedupePref::PreferExisting
        } else {
            mgc_resolver::solver::DedupePref::PreferLatest
        });
    }

    fn set_existing_versions(&self, versions: std::collections::HashMap<String, String>) {
        let parsed: std::collections::HashMap<String, Version> = versions
            .iter()
            .filter_map(|(name, v)| Version::parse(v).ok().map(|ver| (name.clone(), ver)))
            .collect();
        *self.existing_versions_guard() = versions;
        self.resolver.set_existing_versions(parsed);
    }

    async fn parse_manifest(&self, project_root: &Path) -> MgResult<Manifest> {
        parse_manifest(project_root)
    }

    async fn prepare_add(
        &self,
        project_root: &Path,
        name: &PackageName,
        range: Option<&VersionRange>,
        opts: AddOptions,
    ) -> MgResult<mgc_types::adapter::PreparedAdd> {
        self.prepare_add_for_core(project_root, name, range, opts, "web")
            .await
    }

    async fn list(&self, project_root: &Path) -> MgResult<Vec<InstalledPackage>> {
        crate::list::run_list(project_root).await
    }

    async fn audit_fix(&self, project_root: &Path, vulnerable: &[PackageId]) -> MgResult<usize> {
        // P0/F6: the re-resolve inside the fix rides the same gate.
        self.arm_age_gate_for(project_root)?;
        // Fresh resolve (never the lockfile short-circuit): the locked
        // graph pins the vulnerable version — reusing it would "fix"
        // nothing and certify the vuln.
        // (Resolve tươi — short-circuit sẽ ghim version dính lỗ hổng.)
        run_audit_fix(project_root, vulnerable, |m| async move {
            self.resolve_fresh(&m).await
        })
        .await
    }
}
