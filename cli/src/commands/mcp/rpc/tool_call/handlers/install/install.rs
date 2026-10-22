//! MagiCore install tool handler.
//! Xử lý tool cài package MagiCore.

use crate::commands::mcp::JsonRpcRequest;
use serde_json::{Value, json};

pub(in crate::commands::mcp) async fn handle_install(req: &JsonRpcRequest) -> Value {
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

    let frozen = req
        .params
        .as_ref()
        .and_then(|p| p.get("arguments"))
        .and_then(|a| a.get("frozen"))
        .and_then(|f| f.as_bool())
        .unwrap_or(false);

    match crate::commands::install::run(packages.clone(), None, false, false, false, frozen).await {
        Ok(()) => {
            let pkg_list = if packages.is_empty() {
                "all from manifest".to_string()
            } else {
                packages.join(", ")
            };
            json!({
                "content": [{
                    "type": "text",
                    "text": format!(
                        "MagiCore install completed{}\nPackages: {}",
                        if frozen { " (frozen mode)" } else { "" },
                        pkg_list
                    )
                }]
            })
        }
        Err(e) => json!({
            "content": [{
                "type": "text",
                "text": format!("Install failed: {}", e)
            }],
            "isError": true
        }),
    }
}
