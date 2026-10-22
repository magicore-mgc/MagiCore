//! Native NuGet install
//! Install NuGet native

use mgc_resolver::protocols::{NuGetProtocol, RegistryProtocol};
use mgc_types::ResolvedGraph;
use mgc_types::adapter::{InstallCacheMode, InstallSummary};
use std::time::Instant;

use super::super::{content_store, entry_from_package, markers_for, shared_store};
use super::error::NugetNativeInstallError;

/// Native NuGet install: download each nupkg → verify the registration's
/// base64 SHA-512 → import to the mgc CAS (blake3) → materialize the
/// global-packages layout (`{nuget_root}/{id-lower}/{version}/` with the
/// nupkg, extracted contents and `.nupkg.sha512`). No `dotnet restore`
/// spawn — mgc owns resolve/fetch/install (Phase 2).
/// Install NuGet native: tải từng nupkg → verify SHA-512 base64 theo
/// registration → import vào CAS mgc (blake3) → materialize layout
/// global-packages (`{nuget_root}/{id-lower}/{version}/` với nupkg, nội
/// dung giải nén và `.nupkg.sha512`). Không spawn `dotnet restore` — mgc
/// giữ resolve/fetch/install (Phase 2).
pub(in crate::install) async fn install_nuget_native(
    graph: &ResolvedGraph,
    lock_packages: &[mgc_lockfile::Package],
) -> Result<InstallSummary, NugetNativeInstallError> {
    let started = Instant::now();
    let protocol = NuGetProtocol::from_env().await;
    let nuget_root = shared_store::SharedStoreRun::nuget()?.install_root;
    let store = content_store()?;
    let mut added = Vec::with_capacity(graph.packages.len());

    for pkg in &graph.packages {
        let mut entry = entry_from_package(pkg);
        entry.extra_markers = markers_for(pkg, lock_packages);
        let bytes = protocol.download(&entry).await?;
        protocol.verify(&entry, &bytes)?;
        store.import_bytes(&bytes)?;
        protocol.materialize(&entry, &bytes, &nuget_root)?;
        added.push(pkg.id.clone());
    }

    Ok(InstallSummary {
        added,
        bytes_from_cache: 0,
        duration_ms: started.elapsed().as_millis() as u64,
        cache_mode: InstallCacheMode::MgCStore,
    })
}
