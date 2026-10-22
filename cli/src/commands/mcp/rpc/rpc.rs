//! Route JSON-RPC methods to focused response owners.
//! Điều phối method JSON-RPC sang owner phản hồi riêng.

use crate::commands::mcp::{JsonRpcRequest, JsonRpcResponse};

pub(in crate::commands::mcp) async fn handle_rpc_request(req: &JsonRpcRequest) -> JsonRpcResponse {
    match req.method.as_str() {
        "initialize" => super::initialize(req),
        "tools/list" => super::tools_list(req),
        "resources/list" => super::resources_list(req),
        "resources/read" => super::resource_read(req),
        "tools/call" => super::tool_call(req).await,
        _ => super::method_not_found(req),
    }
}
