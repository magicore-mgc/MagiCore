//! Tests for the public AI framework mappings.
//! Test ánh xạ framework AI công khai.

use super::super::AiFramework;

#[test]
fn canonical_ids_and_entry_scripts_remain_stable() {
    assert_eq!(AiFramework::PythonAgent.as_str(), "python-agent");
    assert_eq!(AiFramework::McpServer.as_str(), "mcp-server");
    assert_eq!(AiFramework::PythonAgent.entry_script(), "src/agent.py");
    assert_eq!(AiFramework::McpServer.entry_script(), "server.py");
}
