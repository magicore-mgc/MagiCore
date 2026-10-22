#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Empty layered input does not invoke any registry or materialization lane.
//! Input phân tầng rỗng không gọi registry hay lane materialize.

use crate::native::rn_layers::install_rn_layers;
use mgc_types::{ResolvedGraph, adapter::InstallCacheMode};
use tempfile::TempDir;

#[tokio::test]
async fn empty_graph_returns_empty_store_summary() {
    let temp = TempDir::new().unwrap();
    let summary = install_rn_layers(
        &ResolvedGraph::empty(),
        &[],
        temp.path(),
        Default::default(),
    )
    .await
    .unwrap();
    assert!(summary.added.is_empty());
    assert_eq!(summary.bytes_from_cache, 0);
    assert!(matches!(summary.cache_mode, InstallCacheMode::MgCStore));
}
