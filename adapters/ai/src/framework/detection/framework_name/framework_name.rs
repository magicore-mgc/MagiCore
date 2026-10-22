//! Canonical AI framework string parser.
//! Parser chuỗi framework AI chuẩn.

use super::super::AiFramework;

/// Parse one recognized framework name without normalizing unknown strings.
/// Parse tên framework được nhận diện, không chuẩn hóa chuỗi lạ.
pub(in crate::framework) fn framework_from_str(name: &str) -> Option<AiFramework> {
    match name {
        "python-agent" => Some(AiFramework::PythonAgent),
        "mcp-server" => Some(AiFramework::McpServer),
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
