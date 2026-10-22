//! Validate required RN inputs then resolve JavaScript, Android and iOS in order.
//! Kiểm tra input bắt buộc rồi resolve JavaScript, Android, iOS theo thứ tự.

use crate::native::rn_layers::{
    RNResolution, resolve_android::resolve_android, resolve_ios::resolve_ios,
    resolve_javascript::resolve_javascript,
};
use mgc_types::{Manifest, MgError, MgResult, ResolvedGraph};
use std::path::Path;

/// Resolve all three tiers in their existing order.
/// Resolve cả ba tier theo thứ tự hiện tại.
pub async fn resolve_rn_layers(manifest: &Manifest, project_root: &Path) -> MgResult<RNResolution> {
    ensure_required_ios_lock(project_root)?;

    let mut graph = ResolvedGraph::empty();
    let mut lock_packages = Vec::new();

    let js_graph = resolve_javascript(manifest, project_root).await?;
    graph.packages.extend(js_graph.packages);

    let android = resolve_android(project_root).await?;
    graph.packages.extend(android.graph.packages);
    lock_packages.extend(android.lock_packages);

    let ios = resolve_ios(project_root).await?;
    graph.packages.extend(ios.graph.packages);
    lock_packages.extend(ios.lock_packages);

    Ok(RNResolution {
        graph,
        lock_packages,
    })
}

fn ensure_required_ios_lock(project_root: &Path) -> MgResult<()> {
    if project_root.join("Podfile").is_file() && !project_root.join("Podfile.lock").is_file() {
        return Err(MgError::Unsupported {
            core: "app",
            capability: "react_native_ios_resolution",
            guidance: concat!(
                "Podfile.lock is required to resolve the iOS tier reproducibly; ",
                "run CocoaPods explicitly to create it, then retry MagiCore resolution",
            )
            .to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
