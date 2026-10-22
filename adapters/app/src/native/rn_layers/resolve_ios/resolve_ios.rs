//! Resolve every iOS pod recorded in Podfile.lock and preserve fail-closed gaps.
//! Resolve mọi pod trong Podfile.lock và giữ các nhánh fail-closed.

use crate::native::rn_layers::{RNResolution, resolve_pod::resolve_pod};
use mgc_resolver::protocols::reactnative::{CocoaPodsProtocol, parse_podfile_lock};
use mgc_types::{MgError, MgResult, ResolvedGraph};
use std::path::Path;

/// Resolve iOS from its lock file, or fail closed when a Podfile has no lock.
/// Resolve tier iOS từ lock file hoặc fail-closed nếu Podfile thiếu lock.
pub(in crate::native::rn_layers) async fn resolve_ios(
    project_root: &Path,
) -> MgResult<RNResolution> {
    let mut result = RNResolution {
        graph: ResolvedGraph::empty(),
        lock_packages: Vec::new(),
    };
    let podfile_lock = project_root.join("Podfile.lock");
    if !podfile_lock.is_file() {
        if project_root.join("Podfile").is_file() {
            return Err(mgc_types::capabilities::unsupported_capability(
                "app",
                "resolve",
                concat!(
                    "React Native iOS dependency resolution is unsupported: MagiCore requires a native ",
                    "CocoaPods resolver and lock writer before this tier can be resolved",
                ),
            ));
        }
        return Ok(result);
    }

    let text = std::fs::read_to_string(&podfile_lock)
        .map_err(|e| MgError::Other(format!("read Podfile.lock: {e}")))?;
    let lock = parse_podfile_lock(&text);
    let cocoapods = CocoaPodsProtocol::from_env();
    for pod in &lock.pods {
        let resolved = resolve_pod(pod, &lock, &cocoapods).await?;
        result.graph.packages.push(resolved.graph_package);
        result.lock_packages.push(resolved.lock_package);
    }
    Ok(result)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
