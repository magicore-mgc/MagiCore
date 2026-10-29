//! Standalone artifact fetch for the dotnet lane.
//! Fetch artifact độc lập cho lane dotnet.
//!
//! Uses `NuGetProtocol` — the same protocol `mgc install` uses for this language — so
//! prefetching and installing cannot drift apart.
//! (Dùng `NuGetProtocol` — đúng protocol `mgc install` dùng cho ngôn ngữ này, nên
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
    let protocol = mgc_resolver::protocols::NuGetProtocol::from_env().await;
    fetch_with_protocol("lib", "dotnet", &protocol, graph, store).await
}
