//! Dispatch model queries to the selected registry owner.
//! Điều phối truy vấn model đến owner registry tương ứng.

use crate::registry::{ModelMetadata, Registry};
use mgc_types::MgResult;

/// Query model metadata from a selected registry.
/// Truy vấn metadata model từ registry được chọn.
pub async fn query_model(registry: &Registry, model_id: &str) -> MgResult<ModelMetadata> {
    match registry {
        Registry::HuggingFace => {
            crate::registry::huggingface_query::query_huggingface(model_id).await
        }
        Registry::TensorFlowHub => crate::registry::unsupported_query::query_tfhub(model_id).await,
        Registry::OnnxZoo => crate::registry::unsupported_query::query_onnx(model_id).await,
        Registry::PyTorchHub => crate::registry::unsupported_query::query_pytorch(model_id).await,
        Registry::Local(path) => crate::registry::local_query::query_local(path, model_id).await,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
