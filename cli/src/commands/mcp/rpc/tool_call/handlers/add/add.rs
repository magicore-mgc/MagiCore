//! MagiCore add tool handler.
//! Xử lý tool thêm dependency vào project.

use crate::commands::mcp::JsonRpcRequest;
use serde_json::{Value, json};

pub(in crate::commands::mcp) async fn handle_add(req: &JsonRpcRequest) -> Value {
    // Extract tool arguments from the JSON-RPC request.
    // Trích arguments của tool từ request JSON-RPC.
    let packages = req
        .params
        .as_ref()
        .and_then(|p| p.get("arguments"))
        .and_then(|a| a.get("packages"))
        .and_then(|p| p.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let dev = req
        .params
        .as_ref()
        .and_then(|p| p.get("arguments"))
        .and_then(|a| a.get("dev"))
        .and_then(|d| d.as_bool())
        .unwrap_or(false);

    // Delegated lanes require explicit compatibility-runtime opt-in.
    // Lane delegated yêu cầu bật compatibility-runtime một cách tường minh.
    let compat_runtime = req
        .params
        .as_ref()
        .and_then(|p| p.get("arguments"))
        .and_then(|a| a.get("compat_runtime"))
        .and_then(|c| c.as_str())
        .map(str::to_string);

    if packages.is_empty() {
        json!({
            "content": [{
                "type": "text",
                "text": "mgc_add requires at least one package"
            }],
            "isError": true
        })
    } else {
        match crate::commands::add::run_many(
            packages.clone(),
            None,
            dev,
            false,
            false,
            false,
            false,
            false,
            None,
            compat_runtime,
        )
        .await
        {
            Ok(()) => json!({
                "content": [{
                    "type": "text",
                    "text": format!(
                        "Added {} package(s){}",
                        packages.len(),
                        if dev { " (dev)" } else { "" }
                    )
                }]
            }),
            Err(e) => json!({
                "content": [{
                    "type": "text",
                    "text": format!("Add failed: {}", e)
                }],
                "isError": true
            }),
        }
    }
}
