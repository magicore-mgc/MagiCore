#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Confirm unimplemented providers return typed unsupported errors.
//! Xác nhận provider chưa triển khai trả lỗi unsupported có type.

use crate::registry::unsupported_query::{query_onnx, query_pytorch, query_tfhub};
use mgc_types::MgError;

#[tokio::test]
async fn tfhub_query_fails_closed() {
    let error = query_tfhub("any").await.unwrap_err();
    assert!(matches!(error, MgError::Unsupported { .. }));
}

#[tokio::test]
async fn onnx_query_fails_closed() {
    let error = query_onnx("any").await.unwrap_err();
    assert!(matches!(error, MgError::Unsupported { .. }));
}

#[tokio::test]
async fn pytorch_query_fails_closed() {
    let error = query_pytorch("any").await.unwrap_err();
    assert!(matches!(error, MgError::Unsupported { .. }));
}
