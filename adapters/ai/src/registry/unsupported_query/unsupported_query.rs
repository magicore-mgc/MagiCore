//! Return typed unsupported errors for unimplemented registry clients.
//! Trả lỗi unsupported có type cho registry chưa triển khai client.

use crate::registry::ModelMetadata;
use mgc_types::{MgError, MgResult};

/// TensorFlow Hub has no query client yet; never fabricate metadata.
/// TensorFlow Hub chưa có client truy vấn; không tạo metadata giả.
pub(in crate::registry) async fn query_tfhub(_model_id: &str) -> MgResult<ModelMetadata> {
    Err(MgError::Unsupported {
        core: "ai",
        capability: "model query (tfhub)",
        guidance: "TensorFlow Hub has no query client implemented yet; use a huggingface.co model id or a local path".to_string(),
    })
}

/// ONNX Model Zoo has no query client yet; never fabricate metadata.
/// ONNX Model Zoo chưa có client truy vấn; không tạo metadata giả.
pub(in crate::registry) async fn query_onnx(_model_id: &str) -> MgResult<ModelMetadata> {
    Err(MgError::Unsupported {
        core: "ai",
        capability: "model query (onnx)",
        guidance: "ONNX Model Zoo has no query client implemented yet; use a huggingface.co model id or a local path".to_string(),
    })
}

/// PyTorch Hub has no query client yet; never fabricate metadata.
/// PyTorch Hub chưa có client truy vấn; không tạo metadata giả.
pub(in crate::registry) async fn query_pytorch(_model_id: &str) -> MgResult<ModelMetadata> {
    Err(MgError::Unsupported {
        core: "ai",
        capability: "model query (pytorch)",
        guidance: "PyTorch Hub has no query client implemented yet; use a huggingface.co model id or a local path".to_string(),
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
