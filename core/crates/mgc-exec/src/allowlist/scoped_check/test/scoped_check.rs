use crate::allowlist::check_tool_scoped;

#[test]
fn legacy_scoped_api_still_denies_managers_and_accepts_catalog_tools() {
    assert!(check_tool_scoped("cargo", None).is_ok());
    assert!(
        check_tool_scoped("npm", None)
            .unwrap_err()
            .to_string()
            .contains("permanently forbidden")
    );
}
