//! Standalone artifact fetch for the Lib core, one folder per language.
//! Fetch artifact độc lập cho core Lib, mỗi ngôn ngữ một folder.
//!
//! # Why this exists — Vì sao có module này
//!
//! `LibAdapter::fetch(graph)` used to fail closed for every non-TypeScript
//! language with the guidance "artifacts are fetched by the toolchain itself
//! during its install". That statement was false: `mgc install` for a Rust
//! project downloads `itoa-1.0.18.crate` through `mgc-resolver` into the MagiCore
//! content store — MagiCore owns the download, the toolchain only compiles.
//!
//! The old behaviour made TypeScript the only language with the
//! `artifact-fetcher` capability and blocked any caller that fetched before
//! installing (prefetch, CI warm-up, air-gapped staging). Each language now
//! routes through the same `RegistryProtocol` trio MagiCore already uses inside
//! `install`: `download` → `verify` → `store.import_bytes`.
//! (Trước đây `fetch(graph)` fail-closed cho mọi ngôn ngữ khác TypeScript với lý
//! do "toolchain tự tải" — điều đó SAI: `mgc install` cho Rust tải
//! `itoa-1.0.18.crate` qua `mgc-resolver` vào content store của MagiCore. Hành
//! vi cũ khiến TS là ngôn ngữ duy nhất có capability này và chặn mọi caller
//! fetch trước khi install. Nay mỗi ngôn ngữ dùng đúng bộ ba
//! `download` → `verify` → `import_bytes` mà `install` đã dùng.)

use mgc_resolver::protocols::RegistryProtocol;
use mgc_store::ContentStore;
use mgc_types::{MgError, MgResult, adapter::ResolvedGraph};

/// Download, verify and CAS-import every package in `graph` through `protocol`.
///
/// Fail-closed per package: the first download, digest mismatch or store error
/// aborts the loop. A partial fetch that reported success would leave the store
/// holding some packages while the caller believes all of them landed.
/// (Fail-closed từng package: lỗi tải/digest/store đầu tiên dừng ngay. Fetch một
/// phần mà báo thành công sẽ để store thiếu package trong khi caller tin đã đủ.)
pub(in crate::adapter) async fn fetch_with_protocol<P: RegistryProtocol>(
    core: &str,
    language: &str,
    protocol: &P,
    graph: &ResolvedGraph,
    store: &ContentStore,
) -> MgResult<usize> {
    let mut fetched = 0usize;
    for package in &graph.packages {
        let entry = crate::install::entry_from_package(package);
        let bytes = protocol.download(&entry).await.map_err(|error| {
            MgError::Network(format!(
                "{language} artifact download failed for '{}@{}': {error}",
                entry.name, entry.version
            ))
        })?;
        protocol.verify(&entry, &bytes).map_err(|error| {
            MgError::Integrity(format!(
                "{core}: {language} artifact digest mismatch for '{}@{}': {error}",
                entry.name, entry.version
            ))
        })?;
        store
            .import_bytes(&bytes)
            .map_err(|error| MgError::Store(error.to_string()))?;
        fetched += 1;
    }
    Ok(fetched)
}
