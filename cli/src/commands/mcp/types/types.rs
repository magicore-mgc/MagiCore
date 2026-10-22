//! JSON-RPC wire request and response structures.
//! Định nghĩa kiểu request và response trên wire JSON-RPC.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub(in crate::commands::mcp) struct JsonRpcRequest {
    #[serde(rename = "jsonrpc")]
    _jsonrpc: String,
    pub(in crate::commands::mcp) id: Option<Value>,
    pub(in crate::commands::mcp) method: String,
    #[serde(default)]
    pub(in crate::commands::mcp) params: Option<Value>,
}

#[derive(Debug, Serialize)]
pub(in crate::commands::mcp) struct JsonRpcResponse {
    pub(in crate::commands::mcp) jsonrpc: String,
    pub(in crate::commands::mcp) id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::commands::mcp) result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::commands::mcp) error: Option<Value>,
}
