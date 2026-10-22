//! Construct the existing JSON-RPC method-not-found response.
//! Tạo response JSON-RPC method-not-found như hiện tại

use crate::commands::mcp::{JsonRpcRequest, JsonRpcResponse};
use serde_json::json;

pub(in crate::commands::mcp) fn method_not_found(req: &JsonRpcRequest) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: req.id.clone(),
        result: None,
        error: Some(json!({
            "code": -32601,
            "message": format!("Method not found: {}", req.method)
        })),
    }
}
