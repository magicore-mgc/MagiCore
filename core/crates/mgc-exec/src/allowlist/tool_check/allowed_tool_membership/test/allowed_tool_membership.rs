use super::super::ensure_tool_is_allowed;

#[test]
fn membership_requires_a_reviewed_catalog_entry() {
    assert!(ensure_tool_is_allowed("cargo", "cargo").is_ok());
    let error = ensure_tool_is_allowed("unknown", "unknown").unwrap_err();
    assert!(error.to_string().contains("not on the allowlist"));
}
