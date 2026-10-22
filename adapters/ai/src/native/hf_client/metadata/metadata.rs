//! Owns pure JSON conversion for model info and model-ID lists.
//! Quản lý chuyển JSON thuần sang model info và danh sách model ID.

use crate::native::ModelInfo;
use serde_json::Value;

/// Parse ModelInfo từ JSON của HF API — hàm thuần để test offline.
// (Parse ModelInfo from HF API JSON — pure fn for offline tests.)
pub(crate) fn parse_model_info(json: &Value, fallback_id: &str) -> ModelInfo {
    ModelInfo {
        id: json
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or(fallback_id)
            .to_string(),
        author: json
            .get("author")
            .and_then(Value::as_str)
            .map(str::to_string),
        downloads: json.get("downloads").and_then(Value::as_u64),
        tags: json
            .get("tags")
            .and_then(Value::as_array)
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default(),
        pipeline_tag: json
            .get("pipeline_tag")
            .and_then(Value::as_str)
            .map(str::to_string),
        // HF metadata omits the total size until sibling metadata is available.
        // HF chưa trả tổng dung lượng; chỉ có thể điền khi có metadata của sibling.
        size_bytes: None,
    }
}

/// Parse danh sách model id từ mảng JSON trả về bởi /api/models.
// (Parse model-id list from the /api/models JSON array.)
pub(crate) fn parse_model_ids(json: &Value) -> Vec<String> {
    json.as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.get("id").and_then(Value::as_str).map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
