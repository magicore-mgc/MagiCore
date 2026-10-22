// Focused tests for direct and dotted Python manager names.
// Test tập trung cho tên manager trực tiếp và Python module có dấu chấm.

use super::*;

#[test]
fn recognizes_direct_tool_and_python_internal_module() {
    assert_eq!(
        forbidden_entry_name("/usr/local/bin/npm.cmd", false).as_deref(),
        Some("npm")
    );
    assert_eq!(
        forbidden_entry_name("pip._internal", true).as_deref(),
        Some("pip")
    );
    assert_eq!(forbidden_entry_name("pip._internal", false), None);
}
