use super::*;

#[test]
fn app_dev_tool_allowlist_rejects_unowned_processes() {
    for tool in ["xcodebuild", "swift", "flutter", "gradle"] {
        assert!(
            app_tool_is_allowed(tool),
            "registered tool rejected: {tool}"
        );
    }
    for tool in ["sh", "bash", "npm", "cargo"] {
        assert!(!app_tool_is_allowed(tool), "unowned tool allowed: {tool}");
    }
}
