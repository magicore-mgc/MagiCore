//! Delegate the JavaScript tier to the existing Web/npm dependency pipeline.
//! Ủy quyền tier JavaScript cho pipeline dependency Web/npm hiện có.

use mgc_types::capabilities::DependencyResolver;
use mgc_types::{Manifest, MgError, MgResult, ResolvedGraph};
use std::path::Path;

/// Resolve JavaScript packages through the Web adapter.
/// Resolve package JavaScript qua adapter Web.
pub(in crate::native::rn_layers) async fn resolve_javascript(
    manifest: &Manifest,
    project_root: &Path,
) -> MgResult<ResolvedGraph> {
    let web = mgc_web_adapter::WebAdapter::new()
        .map_err(|e| MgError::Other(format!("web adapter construction failed: {e}")))?;
    web.arm_age_gate_for_core(project_root, "app")?;
    DependencyResolver::resolve(&web, manifest).await
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
