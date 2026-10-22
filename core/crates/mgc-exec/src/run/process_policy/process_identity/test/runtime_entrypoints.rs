//! Existing basename and package-manager entrypoint contract.
//! Contract basename và entrypoint package-manager hiện có.

#[test]
fn monitor_recognizes_runtime_package_manager_entrypoints() {
    for (entry, expected) in [
        ("npm-cli.js", "npm"),
        ("pnpm-cli.cjs", "pnpm"),
        ("yarn-cli.mjs", "yarn"),
        ("bun-cli.js", "bun"),
    ] {
        let command = format!("node.exe C:/project/node_modules/{entry} install");
        assert_eq!(
            super::forbidden_process_name("node.exe", &command, &[]).as_deref(),
            Some(expected)
        );
        assert!(super::forbidden_process_name("node.exe", &command, &[expected]).is_none());
    }
}
