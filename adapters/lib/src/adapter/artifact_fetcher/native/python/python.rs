//! Standalone artifact fetch for the python lane.
//! Fetch artifact độc lập cho lane python.
//!
//! Uses `PypiProtocol` — the same protocol `mgc install` uses for this language — so
//! prefetching and installing cannot drift apart.
//! (Dùng `PypiProtocol` — đúng protocol `mgc install` dùng cho ngôn ngữ này, nên
//! prefetch và install không thể lệch nhau.)

use mgc_store::ContentStore;
use mgc_types::{MgResult, adapter::ResolvedGraph};

use super::super::fetch_with_protocol;

/// Fetch every package in `graph` into the MagiCore content store.
/// Fetch mọi package trong `graph` vào content store MagiCore.
pub(in crate::adapter) async fn fetch(
    graph: &ResolvedGraph,
    store: &ContentStore,
) -> MgResult<usize> {
    let protocol = mgc_resolver::protocols::PypiProtocol::from_env();
    fetch_with_protocol("lib", "python", &protocol, graph, store).await
}
