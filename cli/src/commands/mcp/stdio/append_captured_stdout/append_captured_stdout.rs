//! Append captured stdout to the first response content item.
//! Gắn stdout đã capture vào phần tử content đầu tiên của response.

use crate::commands::mcp::JsonRpcResponse;
use serde_json::Value;

pub(in crate::commands::mcp) fn append_captured_stdout(
    response: &mut JsonRpcResponse,
    output: &str,
) {
    let Some(result) = response.result.as_mut() else {
        return;
    };
    let content_key = if result.get("content").is_some() {
        "content"
    } else {
        "contents"
    };
    let contents = result.get_mut(content_key).and_then(Value::as_array_mut);
    let Some(first_content) = contents.and_then(|items| items.first_mut()) else {
        return;
    };
    let Some(text) = first_content.get_mut("text") else {
        return;
    };
    let existing = text.as_str().unwrap_or_default();
    *text = Value::String(if existing.is_empty() {
        output.to_string()
    } else {
        format!("{existing}\n{output}")
    });
}
