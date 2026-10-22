use crate::allowlist::PROJECT_LANGUAGE_EXECUTABLES;

#[test]
fn runtime_catalog_contains_only_the_scoped_language_set() {
    for runtime in ["bash", "clang", "dotnet", "java", "tsx", "zig"] {
        assert!(PROJECT_LANGUAGE_EXECUTABLES.contains(&runtime));
    }
    for tool in ["npm", "adb", "xcrun", "cargo", "python"] {
        assert!(!PROJECT_LANGUAGE_EXECUTABLES.contains(&tool));
    }
}
