//! Empty-input contract for Go graph resolution.
//! Hợp đồng input rỗng của graph Go.

use super::super::*;

#[tokio::test]
async fn empty_roots_return_an_empty_graph_without_network_requests() {
    let protocol = GoModProtocol::with_sum_base("https://proxy.invalid", "https://sum.invalid");
    assert!(
        protocol
            .resolve_mvs_roots(&[])
            .await
            .expect("test operation must succeed")
            .is_empty()
    );
}
