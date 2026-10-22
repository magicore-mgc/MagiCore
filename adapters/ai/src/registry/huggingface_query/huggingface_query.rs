//! Fetch model metadata from the Hugging Face Hub client.
//! Lấy metadata model thông qua client Hugging Face Hub.

use crate::registry::ModelMetadata;
use crate::registry::metadata_from_hf_json::metadata_from_hf_json;
use mgc_types::MgResult;

/// Query Hugging Face model metadata.
/// Truy vấn metadata model Hugging Face.
pub(in crate::registry) async fn query_huggingface(model_id: &str) -> MgResult<ModelMetadata> {
    let client = crate::native::HuggingFaceClient::new();
    let json = client.get_json(&format!("/models/{model_id}")).await?;
    Ok(metadata_from_hf_json(&json, model_id))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
