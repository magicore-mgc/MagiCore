use super::super::validate_non_empty_tool_name;

#[test]
fn trims_names_and_rejects_whitespace_only_values() {
    assert_eq!(validate_non_empty_tool_name(" cargo ").unwrap(), "cargo");
    assert_eq!(
        validate_non_empty_tool_name("  ").unwrap_err().to_string(),
        "tool name is empty"
    );
}
