//! Coordinate Swift registry installs and fail-closed Git guards.
//! Điều phối install Swift registry và guard Git fail-closed.
use super::{
    SwiftPackageSource, build_swift_pins, git_commit_for_package, install_registry_archive,
    materialize_git_checkout, swift_package_source, swift_store_root,
};
use crate::install::content_store;
use mgc_resolver::protocols::SwiftRegistryProtocol;
use mgc_types::adapter::{InstallCacheMode, InstallSummary};
use mgc_types::{MgResult, ResolvedGraph};
use std::path::Path;

/// Native Swift install downloads, verifies, CAS-imports and extracts registry archives.
/// Git/local inputs follow a defensive protocol route that currently returns Unsupported;
/// MagiCore does not spawn Git or fall back to an unverified checkout. A SwiftPM-compatible
/// Package.resolved (v2) is exported for graph entries. SwiftPM is not spawned.
/// Install Swift native tải, xác minh, import CAS rồi giải nén archive registry.
/// Input Git/local đi vào route protocol phòng thủ hiện trả Unsupported; MagiCore không
/// chạy Git và không fallback sang checkout chưa xác minh. Package.resolved (v2) tương
/// thích SwiftPM được xuất cho graph entry. SwiftPM không được spawn.
pub(crate) async fn install_swift_native(
    graph: &ResolvedGraph,
    lock_packages: &[mgc_lockfile::Package],
    project_root: &Path,
    protocol: Option<SwiftRegistryProtocol>,
) -> MgResult<InstallSummary> {
    let started = std::time::Instant::now();
    let protocol = protocol.unwrap_or_else(SwiftRegistryProtocol::from_env);
    let swift_root = swift_store_root()?;
    let store = content_store()?;
    let mut added = Vec::with_capacity(graph.packages.len());

    for package in &graph.packages {
        match swift_package_source(package) {
            SwiftPackageSource::GitCheckout => {
                // Git/local inputs stay fail-closed until native transport exists; no child process runs.
                // Nguồn Git/local tiếp tục bị chặn tới khi có transport native; không chạy process con.
                let entry = crate::install::entry_from_package(package);
                let expected_commit = git_commit_for_package(lock_packages, package.id.name_str());
                materialize_git_checkout(
                    &protocol,
                    &entry,
                    expected_commit.as_deref(),
                    &swift_root,
                )?;
            }
            SwiftPackageSource::RegistryArchive => {
                let entry = crate::install::entry_from_package(package);
                install_registry_archive(&protocol, &entry, &store, &swift_root).await?;
            }
        }
        added.push(package.id.clone());
    }

    // Export the SwiftPM-compatible Package.resolved from the resolution.
    // (Xuất Package.resolved tương thích SwiftPM từ kết quả resolve.)
    let pins = build_swift_pins(graph, lock_packages);
    protocol.export_package_resolved(&pins, project_root)?;

    Ok(InstallSummary {
        added,
        bytes_from_cache: 0,
        duration_ms: started.elapsed().as_millis() as u64,
        cache_mode: InstallCacheMode::MgCStore,
    })
}
