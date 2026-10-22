//! Prepare one dependency through native resolve-first before the install transaction.
//! Chuẩn bị một dependency qua resolve-first native trước transaction install.

use crate::adapter::dependency_resolver::stage_pending_lock;
use crate::adapter::native_registry::{NativeRegistry, NativeRegistryOperation};
use crate::adapter::types::LibAdapter;
use mgc_types::PackageAdapter;
use mgc_types::adapter::{AddOptions, PreparedAdd};
use mgc_types::{
    DependencySpec, Ecosystem, Manifest, MgResult, PackageId, PackageName, VersionRange,
};
use std::path::Path;

use super::pinning::pin_requested_range;

impl LibAdapter {
    /// Prepare ONE dependency natively — the protocol engine selects the
    /// real version and stages lock entries for the install tail. No file is
    /// written here; the CLI mutation gateway commits it under journal/lock.
    /// (Resolve native một dependency — không bao giờ spawn toolchain.)
    pub(in crate::adapter) async fn resolve_first_add(
        &self,
        project_root: &Path,
        name: &PackageName,
        range: Option<&VersionRange>,
        opts: &AddOptions,
    ) -> MgResult<Option<PreparedAdd>> {
        if let Some(web) = &self.web {
            web.arm_age_gate_for(project_root)?;
        } else {
            self.ensure_age_gate_supported(project_root)?;
        }
        let Some(kind) = self.resolve_first_lane()? else {
            return Ok(None);
        };
        // Scratch carries the REQUESTED range (star when the user gave
        // none) — the engine selects within it; the saved range stays
        // pinned only when the request was unpinned.
        let wanted: VersionRange = range.cloned().unwrap_or_else(VersionRange::star);
        let mut scratch = Manifest::new("scratch", Ecosystem::Lib);
        scratch.add_dep(
            DependencySpec::new(name.clone(), wanted.clone()),
            opts.dev,
            opts.optional,
            opts.peer,
        );
        let registry = NativeRegistry::for_lane(
            self,
            project_root,
            &kind,
            NativeRegistryOperation::SinglePackageAdd,
        )
        .await?;
        let resolution = registry.resolve(&scratch, opts.dev).await?;
        let resolved = resolution
            .graph
            .packages
            .iter()
            .find(|p| p.id.name_str() == name.as_str())
            .ok_or_else(|| {
                mgc_types::MgError::Other(format!(
                    "native resolve returned no entry for '{}' — refusing to book an unresolved dep",
                    name.as_str()
                ))
            })?;
        let pinned = pin_requested_range(&kind, wanted, resolved.id.version())?;
        stage_pending_lock(&self.pending_lock, resolution.lock_packages)?;
        Ok(Some(PreparedAdd {
            id: PackageId::new(name.clone(), resolved.id.version().clone()),
            range: pinned,
        }))
    }
}
