#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Tests that the public query route reaches the selected owner.
//! Test route truy vấn công khai gọi đúng owner được chọn.

use crate::registry::{Registry, query_model};
use mgc_types::MgError;

#[tokio::test]
async fn query_dispatch_preserves_unsupported_provider_error() {
    let error = query_model(&Registry::TensorFlowHub, "model")
        .await
        .unwrap_err();
    assert!(matches!(error, MgError::Unsupported { .. }));
}
