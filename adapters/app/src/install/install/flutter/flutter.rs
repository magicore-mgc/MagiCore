//! Run the native Flutter install and preserve its lock-write policy.
//! Chạy install Flutter native và giữ policy ghi lock hiện tại.

use crate::install::{direct_package_ids, install_flutter_native, write_canonical_lock_with_roots};
use mgc_types::adapter::InstallSummary;
use mgc_types::{MgResult, ResolvedGraph};
use std::path::Path;

pub(in crate::install::install) async fn run_flutter_install(
    graph: &ResolvedGraph,
    project_root: &Path,
    offline: bool,
    replay_from_lock: bool,
    lock_packages: Vec<mgc_lockfile::Package>,
) -> MgResult<InstallSummary> {
    let summary = install_flutter_native(graph, project_root, offline).await?;
    if should_write_flutter_lock(offline, replay_from_lock) {
        write_canonical_lock_with_roots(
            project_root,
            &[mgc_lockfile::EcosystemTag::Dart],
            lock_packages,
            direct_package_ids(graph),
        )?;
    }
    Ok(summary)
}

fn should_write_flutter_lock(offline: bool, replay_from_lock: bool) -> bool {
    !offline && !replay_from_lock
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
