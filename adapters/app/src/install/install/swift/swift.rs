//! Run the native Swift install and preserve its lock-write policy.
//! Chạy install Swift native và giữ policy ghi lock hiện tại.

use crate::install::{direct_package_ids, install_swift_native, write_canonical_lock_with_roots};
use mgc_resolver::protocols::SwiftRegistryProtocol;
use mgc_types::adapter::InstallSummary;
use mgc_types::{MgResult, ResolvedGraph};
use std::path::Path;

pub(in crate::install::install) async fn run_swift_install(
    graph: &ResolvedGraph,
    project_root: &Path,
    replay_from_lock: bool,
    lock_packages: Vec<mgc_lockfile::Package>,
    swift_protocol: Option<SwiftRegistryProtocol>,
) -> MgResult<InstallSummary> {
    let summary = install_swift_native(graph, &lock_packages, project_root, swift_protocol).await?;
    if should_write_swift_lock(replay_from_lock) {
        write_canonical_lock_with_roots(
            project_root,
            &[mgc_lockfile::EcosystemTag::Swift],
            lock_packages,
            direct_package_ids(graph),
        )?;
    }
    Ok(summary)
}

fn should_write_swift_lock(replay_from_lock: bool) -> bool {
    !replay_from_lock
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
