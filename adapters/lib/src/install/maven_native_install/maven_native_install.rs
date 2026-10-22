//! Native Maven install
//! Install Maven native

use mgc_resolver::protocols::{MavenProtocol, RegistryProtocol};
use mgc_types::ResolvedGraph;
use mgc_types::adapter::{InstallCacheMode, InstallSummary};
use std::time::Instant;

use super::super::{content_store, entry_from_package, lock_entry_for, markers_for, shared_store};
use super::error::MavenNativeInstallError;

/// Native Maven install: download each jar → verify sha256 (or the recorded
/// sha1) → import to the mgc CAS (blake3) → download the POM → materialize
/// into `{m2_root}/repository/{gpath}/{artifact}/{version}/` (`mvn -o`
/// readable). No `mvn dependency:go-offline` spawn — mgc owns
/// resolve/fetch/install for pom.xml projects (Phase 2).
/// Install Maven native: tải từng jar → verify sha256 (hoặc sha1 đã ghi) →
/// import vào CAS mgc (blake3) → tải POM → materialize vào
/// `{m2_root}/repository/{gpath}/{artifact}/{version}/` (đọc được bởi
/// `mvn -o`). Không spawn `mvn dependency:go-offline` — mgc giữ
/// resolve/fetch/install cho project pom.xml (Phase 2).
pub(in crate::install) async fn install_maven_native(
    graph: &ResolvedGraph,
    lock_packages: &[mgc_lockfile::Package],
) -> Result<InstallSummary, MavenNativeInstallError> {
    let started = Instant::now();
    let protocol = MavenProtocol::from_env();
    let m2_root = shared_store::SharedStoreRun::maven()?.install_root;
    let store = content_store()?;
    let mut added = Vec::with_capacity(graph.packages.len());

    for pkg in &graph.packages {
        let mut entry = entry_from_package(pkg);
        entry.extra_markers = markers_for(pkg, lock_packages);
        // Maven coordinates keep their RAW version string (`1.3`, never
        // normalized `1.3.0`): filenames on Central are literal. Prefer
        // the lock entry's raw version when present.
        // (Tọa độ Maven giữ chuỗi version THÔ.)
        if let Some(raw) = lock_entry_for(pkg, lock_packages).map(|p| p.version.clone()) {
            entry.version = raw;
        }
        let jar = protocol.download(&entry).await?;
        protocol.verify(&entry, &jar)?;
        let (group, artifact) = MavenProtocol::split_coordinate(&entry.name)?;
        let pom = protocol
            .download_pom(&group, &artifact, &entry.version)
            .await?;
        store.import_bytes(&jar)?;
        store.import_bytes(&pom)?;
        protocol.materialize(&entry, &jar, &pom, &m2_root)?;
        added.push(pkg.id.clone());
    }

    Ok(InstallSummary {
        added,
        bytes_from_cache: 0,
        duration_ms: started.elapsed().as_millis() as u64,
        cache_mode: InstallCacheMode::MgCStore,
    })
}
