use crate::allowlist::normalize_script_token;

#[test]
fn normalizes_windows_and_script_entrypoint_suffixes() {
    assert_eq!(
        normalize_script_token(r"C:\tools\NPM.CMD").as_deref(),
        Some("npm")
    );
    assert_eq!(
        normalize_script_token("./node_modules/pnpm/bin/pnpm.cjs").as_deref(),
        Some("pnpm")
    );
    assert_eq!(
        normalize_script_token(r#"  "YARN-CLI.JS"  "#).as_deref(),
        Some("yarn")
    );
    assert_eq!(normalize_script_token("  "), None);
}
