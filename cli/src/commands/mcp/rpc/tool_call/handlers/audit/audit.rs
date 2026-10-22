//! MagiCore audit tool handler.
//! Xử lý tool audit bảo mật.

use crate::commands::mcp::JsonRpcRequest;
use serde_json::{Value, json};

pub(in crate::commands::mcp) async fn handle_audit(req: &JsonRpcRequest) -> Value {
    let fix = req
        .params
        .as_ref()
        .and_then(|p| p.get("arguments"))
        .and_then(|a| a.get("fix"))
        .and_then(|f| f.as_bool())
        .unwrap_or(false);

    match crate::commands::audit::run(None, fix, None).await {
        Ok(()) => json!({
            "content": [{
                "type": "text",
                "text": format!(
                    "Security audit completed{}",
                    if fix { " (auto-fix applied)" } else { "" }
                )
            }]
        }),
        Err(e) => json!({
            "content": [{
                "type": "text",
                "text": format!("Audit failed: {}", e)
            }],
            "isError": true
        }),
    }
}
