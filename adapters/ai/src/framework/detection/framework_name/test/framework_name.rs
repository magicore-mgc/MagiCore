//! Tests for the canonical AI framework-name parser.
//! Test parser tên framework AI chuẩn.

use super::super::framework_from_str;
use crate::framework::AiFramework;

#[test]
fn maps_the_existing_canonical_names() {
    assert_eq!(
        framework_from_str("python-agent"),
        Some(AiFramework::PythonAgent)
    );
    assert_eq!(
        framework_from_str("mcp-server"),
        Some(AiFramework::McpServer)
    );
}

#[test]
fn unknown_framework_name_returns_none() {
    assert_eq!(framework_from_str("other"), None);
}
