//! MagiCore versions tool handler.
//! Xử lý tool tra cứu phiên bản package.

use crate::commands::mcp::JsonRpcRequest;
use serde_json::{Value, json};

pub(in crate::commands::mcp) async fn handle_versions(req: &JsonRpcRequest) -> Value {
    let package = req
        .params
        .as_ref()
        .and_then(|params| params.get("arguments"))
        .and_then(|args| args.get("package"))
        .and_then(Value::as_str)
        .unwrap_or_default();
    if package.trim().is_empty() {
        json!({
            "content": [{"type": "text", "text": "mgc_versions requires a package name"}],
            "isError": true
        })
    } else {
        match crate::commands::info::versions_json(package).await {
            Ok(text) => json!({"content": [{"type": "text", "text": text}]}),
            Err(error) => json!({
                "content": [{"type": "text", "text": format!("Version query failed: {error:#}")}],
                "isError": true
            }),
        }
    }
}
