//! Lib dependency-resolver trait facade.
//! Facade trait resolver dependency của Lib.

use crate::adapter::types::LibAdapter;
use async_trait::async_trait;
use mgc_types::capabilities::DependencyResolver;
use mgc_types::{Manifest, MgResult, ResolvedGraph};

#[async_trait]
impl DependencyResolver for LibAdapter {
    /// Direct adapter mutations remain fail-closed; the CLI gateway owns dependency writes.
    /// Thao tác trực tiếp vẫn fail-closed; CLI gateway sở hữu việc ghi dependency.
    fn probe_dependency_resolver(&self) -> MgResult<()> {
        self.probe_native_dependency_resolver()
    }

    async fn resolve(&self, manifest: &Manifest) -> MgResult<ResolvedGraph> {
        self.resolve_native_dependencies(manifest).await
    }

    /// Resolve without the Web delegate's lock short-circuit.
    /// Resolve không dùng short-circuit lock của Web delegate.
    async fn resolve_fresh(&self, manifest: &Manifest) -> MgResult<ResolvedGraph> {
        self.resolve_native_dependencies_fresh(manifest).await
    }
}
