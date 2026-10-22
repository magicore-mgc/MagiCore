//! MagiCore redacted config tool handler.
//! Xử lý tool đọc config đã che dữ liệu nhạy cảm.

use serde_json::{Value, json};

pub(in crate::commands::mcp) fn handle_config() -> Value {
    match crate::commands::config::list_local_redacted() {
        Ok(text) => json!({"content": [{"type": "text", "text": text}]}),
        Err(error) => json!({
            "content": [{"type": "text", "text": format!("Config read failed: {error:#}")}],
            "isError": true
        }),
    }
}
