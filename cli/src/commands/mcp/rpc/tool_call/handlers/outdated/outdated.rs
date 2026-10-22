//! MagiCore outdated tool handler.
//! Xử lý tool kiểm tra package đã lỗi thời.

use serde_json::{Value, json};

pub(in crate::commands::mcp) async fn handle_outdated() -> Value {
    match crate::commands::outdated::outdated_json(None).await {
        Ok(text) => json!({"content": [{"type": "text", "text": text}]}),
        Err(error) => json!({
            "content": [{"type": "text", "text": format!("Outdated query failed: {error:#}")}],
            "isError": true
        }),
    }
}
