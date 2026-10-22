//! Initialize MCP protocol capabilities and server information.
//! Khởi tạo capability giao thức và thông tin server MCP.

use crate::commands::mcp::{JsonRpcRequest, JsonRpcResponse};
use serde_json::json;

pub(in crate::commands::mcp) fn initialize(req: &JsonRpcRequest) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: req.id.clone(),
        result: Some(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {},
                "resources": { "listChanged": false }
            },
            "serverInfo": {
                "name": "magicore-native-mcp",
                "version": env!("CARGO_PKG_VERSION")
            }
        })),
        error: None,
    }
}
