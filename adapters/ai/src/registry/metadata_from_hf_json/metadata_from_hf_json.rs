//! Map Hugging Face model JSON into stable adapter metadata.
//! Ánh xạ JSON model Hugging Face sang metadata ổn định của adapter.

use crate::registry::{ModelFormat, ModelMetadata, Registry};

/// Project the `/api/models/{id}` response fields used by this adapter.
/// Chuyển các trường `/api/models/{id}` mà adapter đang sử dụng.
pub(in crate::registry) fn metadata_from_hf_json(
    json: &serde_json::Value,
    fallback_id: &str,
) -> ModelMetadata {
    ModelMetadata {
        id: json
            .get("id")
            .and_then(|value| value.as_str())
            .unwrap_or(fallback_id)
            .to_string(),
        registry: Registry::HuggingFace,
        // "sha" is the Hub commit revision; missing it retains main.
        // SHA của Hub là revision commit; thiếu SHA thì giữ main.
        version: json
            .get("sha")
            .and_then(|value| value.as_str())
            .map(str::to_string)
            .or_else(|| Some("main".into())),
        size_bytes: None,
        format: Some(ModelFormat::HuggingFace),
        tags: json
            .get("tags")
            .and_then(|value| value.as_array())
            .map(|values| {
                values
                    .iter()
                    .filter_map(|value| value.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default(),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
