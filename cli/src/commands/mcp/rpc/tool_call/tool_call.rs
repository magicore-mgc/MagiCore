//! Route tools/call requests and preserve the JSON-RPC envelope.
//! Điều phối tools/call và giữ nguyên cấu trúc phản hồi JSON-RPC.

use crate::commands::mcp::{JsonRpcRequest, JsonRpcResponse};

pub(in crate::commands::mcp) async fn tool_call(req: &JsonRpcRequest) -> JsonRpcResponse {
    let tool_name = req
        .params
        .as_ref()
        .and_then(|params| params.get("name"))
        .and_then(|name| name.as_str())
        .unwrap_or_default();
    let tool_res = super::handlers::dispatch(tool_name, req).await;
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: req.id.clone(),
        result: Some(tool_res),
        error: None,
    }
}
