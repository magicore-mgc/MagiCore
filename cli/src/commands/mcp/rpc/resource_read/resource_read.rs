//! Read workspace, capability or redacted local configuration content.
//! Đọc workspace, capability hoặc nội dung cấu hình local đã che bí mật.

use crate::commands::mcp::{JsonRpcRequest, JsonRpcResponse};
use serde_json::{Value, json};

pub(in crate::commands::mcp) fn resource_read(req: &JsonRpcRequest) -> JsonRpcResponse {
    let uri = req
        .params
        .as_ref()
        .and_then(|params| params.get("uri"))
        .and_then(Value::as_str)
        .unwrap_or_default();
    let content = match uri {
        "mgc://workspace" => crate::commands::workspace::workspace_info_json(),
        "mgc://capabilities" => crate::commands::capabilities::json_payload(None)
            .and_then(|payload| serde_json::to_string_pretty(&payload).map_err(Into::into)),
        "mgc://config/local" => crate::commands::config::list_local_redacted(),
        _ => Err(anyhow::anyhow!("Unknown resource: {uri}")),
    };
    match content {
        Ok(text) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: req.id.clone(),
            result: Some(json!({
                "contents": [{
                    "uri": uri,
                    "mimeType": if uri == "mgc://config/local" {
                        "text/plain"
                    } else {
                        "application/json"
                    },
                    "text": text
                }]
            })),
            error: None,
        },
        Err(error) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: req.id.clone(),
            result: None,
            error: Some(json!({ "code": -32002, "message": error.to_string() })),
        },
    }
}
