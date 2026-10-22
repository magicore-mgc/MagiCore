//! List the existing workspace, capability and redacted-config resources.
//! Trả danh sách resource workspace, capability và config đã che bí mật.

use crate::commands::mcp::{JsonRpcRequest, JsonRpcResponse};
use serde_json::json;

pub(in crate::commands::mcp) fn resources_list(req: &JsonRpcRequest) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: req.id.clone(),
        result: Some(json!({
            "resources": [
                {
                    "uri": "mgc://workspace",
                    "name": "Workspace graph",
                    "description": "Workspace packages and dependency edges",
                    "mimeType": "application/json"
                },
                {
                    "uri": "mgc://capabilities",
                    "name": "Core capabilities",
                    "description": "Machine-readable MagiCore core capability manifest",
                    "mimeType": "application/json"
                },
                {
                    "uri": "mgc://config/local",
                    "name": "Local configuration",
                    "description": "Project configuration with sensitive values redacted",
                    "mimeType": "text/plain"
                }
            ]
        })),
        error: None,
    }
}
