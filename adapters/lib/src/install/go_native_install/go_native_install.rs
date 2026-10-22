//! Native Go install
//! Install Go native

use mgc_resolver::protocols::{GoModProtocol, RegistryProtocol};
use mgc_types::ResolvedGraph;
use mgc_types::adapter::{InstallCacheMode, InstallSummary};
use std::time::Instant;

use super::super::{content_store, entry_from_package, markers_for, shared_store};
use super::error::GoNativeInstallError;

/// Native Go install: download the module set (zip + go.mod + .info) →
/// verify sha256 (proxy .ziphash) or the sumdb `h1:` directory hash →
/// import to the mgc CAS (blake3) → materialize the go download cache
/// (`{gomodcache}/cache/download/{module}/@v/…`, `GOPROXY=off` readable).
/// No `go mod download` spawn — mgc owns resolve/fetch/install (Phase 2).
/// Install Go native: tải trọn bộ module (zip + go.mod + .info) → xác minh
/// sha256 (proxy .ziphash) hoặc directory hash `h1:` của sumdb → import vào
/// CAS mgc (blake3) → materialize go download cache
/// (`{gomodcache}/cache/download/{module}/@v/…`, đọc được với `GOPROXY=off`).
/// Không spawn `go mod download` — mgc giữ resolve/fetch/install (Phase 2).
pub(in crate::install) async fn install_go_native(
    graph: &ResolvedGraph,
    lock_packages: &[mgc_lockfile::Package],
) -> Result<InstallSummary, GoNativeInstallError> {
    let started = Instant::now();
    let protocol = GoModProtocol::from_env();
    let gomodcache = shared_store::SharedStoreRun::go()?.install_root;
    let store = content_store()?;
    let mut added = Vec::with_capacity(graph.packages.len());

    for pkg in &graph.packages {
        let mut entry = entry_from_package(pkg);
        entry.extra_markers = markers_for(pkg, lock_packages);
        let files = protocol.download_module(&entry).await?;
        protocol.verify(&entry, &files.zip)?;
        store.import_bytes(&files.zip)?;
        store.import_bytes(&files.gomod)?;
        store.import_bytes(&files.info)?;
        protocol.materialize(&entry, &files, &gomodcache)?;
        added.push(pkg.id.clone());
    }

    Ok(InstallSummary {
        added,
        bytes_from_cache: 0,
        duration_ms: started.elapsed().as_millis() as u64,
        cache_mode: InstallCacheMode::MgCStore,
    })
}
