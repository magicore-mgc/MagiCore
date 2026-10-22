//! Dispatch MCP tool names to their native handlers.
//! Điều phối tên tool MCP sang handler MagiCore tương ứng

use super::{
    handle_add, handle_audit, handle_config, handle_install, handle_outdated, handle_uninstall,
    handle_versions, handle_workspace_info,
};
use crate::commands::mcp::JsonRpcRequest;
use serde_json::{Value, json};

pub(in crate::commands::mcp) async fn dispatch(tool_name: &str, req: &JsonRpcRequest) -> Value {
    match tool_name {
        "mgc_install" => handle_install(req).await,
        "mgc_uninstall" => handle_uninstall(req).await,
        "mgc_versions" => handle_versions(req).await,
        "mgc_outdated" => handle_outdated().await,
        "mgc_config" => handle_config(),
        "mgc_audit" => handle_audit(req).await,
        "mgc_workspace_info" => handle_workspace_info(),
        "mgc_add" => handle_add(req).await,
        _ => {
            json!({"content": [{"type": "text", "text": format!("Unknown tool: {tool_name}")}], "isError": true})
        }
    }
}
