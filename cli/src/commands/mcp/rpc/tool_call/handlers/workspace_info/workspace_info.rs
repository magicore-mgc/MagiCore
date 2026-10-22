//! MagiCore workspace information tool handler.
//! Xử lý tool đọc đồ thị workspace.

use serde_json::{Value, json};

pub(in crate::commands::mcp) fn handle_workspace_info() -> Value {
    match crate::commands::workspace::workspace_info_json() {
        Ok(command_output) => json!({
            "content": [{
                "type": "text",
                "text": command_output
            }]
        }),
        Err(e) => json!({
            "content": [{
                "type": "text",
                "text": format!("Workspace query failed: {}", e)
            }],
            "isError": true
        }),
    }
}
