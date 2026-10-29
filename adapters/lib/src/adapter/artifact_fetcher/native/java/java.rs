//! Standalone artifact fetch for the java lane.
//! Fetch artifact độc lập cho lane java.
//!
//! Uses `MavenProtocol` — the same protocol `mgc install` uses for this language — so
//! prefetching and installing cannot drift apart.
//! (Dùng `MavenProtocol` — đúng protocol `mgc install` dùng cho ngôn ngữ này, nên
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
    let protocol = mgc_resolver::protocols::MavenProtocol::from_env();
    fetch_with_protocol("lib", "java", &protocol, graph, store).await
}
