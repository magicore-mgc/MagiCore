//! MagiCore uninstall tool handler.
//! Xử lý tool gỡ package MagiCore.

use crate::commands::mcp::JsonRpcRequest;
use serde_json::{Value, json};

pub(in crate::commands::mcp) async fn handle_uninstall(req: &JsonRpcRequest) -> Value {
    let arguments = req
        .params
        .as_ref()
        .and_then(|params| params.get("arguments"));
    let packages = arguments
        .and_then(|args| args.get("packages"))
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let compat_runtime = arguments
        .and_then(|args| args.get("compat_runtime"))
        .and_then(Value::as_str);
    if packages.is_empty() {
        json!({
            "content": [{"type": "text", "text": "mgc_uninstall requires at least one package"}],
            "isError": true
        })
    } else {
        match crate::commands::remove::run_many(
            packages.clone(),
            None,
            compat_runtime.map(str::to_string),
        )
        .await
        {
            Ok(()) => json!({
                "content": [{"type": "text", "text": format!("Removed {} package(s)", packages.len())}]
            }),
            Err(error) => json!({
                "content": [{"type": "text", "text": format!("Uninstall failed: {error:#}")}],
                "isError": true
            }),
        }
    }
}
