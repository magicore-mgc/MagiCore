#![cfg(test)]

//! Ensure the public result keeps its graph and lock fields constructible.
//! Xác nhận field graph và lock công khai vẫn khởi tạo được.

use crate::native::rn_layers::RNResolution;
use mgc_types::ResolvedGraph;

#[test]
fn resolution_result_keeps_public_fields() {
    let resolution = RNResolution {
        graph: ResolvedGraph::empty(),
        lock_packages: Vec::new(),
    };
    assert!(resolution.graph.packages.is_empty());
    assert!(resolution.lock_packages.is_empty());
}
