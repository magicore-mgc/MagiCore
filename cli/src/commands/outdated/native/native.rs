//! Resolves fresh native candidates from the verified project lock.
//! Resolve candidate native mới từ lock project đã kiểm chứng.

use super::super::model::OutdatedPkg;
use super::{
    candidate_manifest::latest_candidate_manifest, graph_compare::native_outdated_packages,
};
use crate::context::ProjectContext;
use anyhow::Result;

pub(in crate::commands::outdated) async fn query_outdated_native(
    ctx: &ProjectContext,
) -> Result<Vec<OutdatedPkg>> {
    let adapter = ctx.adapter();
    adapter.probe_dependency_resolver()?;
    adapter.arm_age_gate_for(ctx.root())?;

    let manifest = adapter.parse_manifest(ctx.root()).await?;
    if manifest.all_dependencies().next().is_none() {
        return Ok(Vec::new());
    }

    let current =
        crate::commands::core::shared::load_locked_graph(ctx.root(), adapter.name(), &manifest)?
            .ok_or_else(|| crate::error::outdated_lock_unusable(adapter.core_id()))?;
    // Validate configured age policy; the fresh resolve never trusts lock timestamps.
    // Xác thực age policy; resolve tươi không dựa vào timestamp trong lock.
    crate::commands::core::shared::refresh_locked_graph_for_age_gate(
        adapter,
        ctx.root(),
        true,
        false,
    )?;
    let latest_manifest = latest_candidate_manifest(&manifest);
    let latest = adapter.resolve_fresh(&latest_manifest).await?;
    Ok(native_outdated_packages(&manifest, &current, &latest)?)
}

#[cfg(all(test, feature = "lib"))]
#[path = "test/mod.rs"]
mod test;
