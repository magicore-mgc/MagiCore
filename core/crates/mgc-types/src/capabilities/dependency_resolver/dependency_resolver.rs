//! Dependency resolver capability implementation.
//! Triển khai capability resolve dependency.

use super::super::{CoreIdent, unsupported_capability};
use crate::adapter::{AddOptions, ResolvedGraph, UpdatedPackage};
use crate::error::MgResult;
use crate::manifest::Manifest;
use crate::package::{PackageId, PackageName, VersionRange};
use async_trait::async_trait;
use std::path::Path;

/// Resolves dependency graphs. Mutations must be committed only by the CLI's
/// journaled mutation gateway; adapter-level `add/remove/update` deliberately
/// fail closed so callers cannot bypass project locking, recovery, and rollback.
/// Resolve graph dependency. Mutation chỉ được commit qua gateway có journal
/// của CLI; `add/remove/update` ở adapter cố ý fail-closed để không bypass lock,
/// recovery và rollback cấp project.
#[async_trait]
pub trait DependencyResolver: CoreIdent {
    /// Lightweight gate probe — never touches the network.
    /// Probe nhẹ cho gate — không bao giờ chạm network.
    fn probe_dependency_resolver(&self) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "resolve",
            "this core has no native registry graph for this project; package-manager fallback is disabled",
        ))
    }

    async fn resolve(&self, _manifest: &Manifest) -> MgResult<ResolvedGraph> {
        Err(unsupported_capability(
            self.core_id(),
            "resolve",
            "this core has no native registry graph for this project; package-manager fallback is disabled",
        ))
    }

    /// Fresh resolve that MUST bypass any lockfile short-circuit: used by
    /// audit --fix (and any bumper), where reusing the locked graph would
    /// pin the vulnerable version forever. Default is plain resolve
    /// (adapters without a short-circuit need nothing special).
    /// (Resolve tươi, bỏ qua short-circuit lockfile — cho audit --fix.)
    async fn resolve_fresh(&self, manifest: &Manifest) -> MgResult<ResolvedGraph> {
        self.resolve(manifest).await
    }

    async fn add(
        &self,
        _project_root: &Path,
        _name: &PackageName,
        _range: Option<&VersionRange>,
        _opts: AddOptions,
    ) -> MgResult<PackageId> {
        Err(unsupported_capability(
            self.core_id(),
            "add",
            "direct adapter mutation is disabled; use the MagiCore CLI mutation gateway when this ecosystem has native support",
        ))
    }

    async fn remove(&self, _project_root: &Path, _name: &PackageName) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "remove",
            "direct adapter mutation is disabled; use the MagiCore CLI mutation gateway when this ecosystem has native support",
        ))
    }

    async fn update(
        &self,
        _project_root: &Path,
        _name: Option<&PackageName>,
    ) -> MgResult<Vec<UpdatedPackage>> {
        Err(unsupported_capability(
            self.core_id(),
            "update",
            "direct adapter mutation is disabled; use the MagiCore CLI mutation gateway when this ecosystem has native support",
        ))
    }
}
